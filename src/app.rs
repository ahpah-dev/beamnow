//! Application state, the agent event pump, and the window layout.

use crate::agent::{self, Event, Job};
use crate::catalog::{self, FreeModel, Wire};
use crate::config::{Config, Selection};
use crate::llm::{Endpoint, Msg};
use crate::session::{self, Entry, Session};
use crate::theme::{self, Palette, RawPalette, ThemePreset};
use crate::views;
use eframe::egui::{self, KeyboardShortcut, Modifiers};
use egui_commonmark::CommonMarkCache;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
pub enum Page {
    Chat,
    Settings,
}

/// The one agent job that may run at a time.
pub struct Running {
    pub session_id: String,
    pub events: Receiver<Event>,
    /// Pending permission request: description and the reply channel.
    pub approval: Option<(String, Sender<bool>)>,
}

pub struct BeamApp {
    pub(crate) cfg: Config,
    pub(crate) palette: Palette,
    pub(crate) page: Page,
    /// Newest first.
    pub(crate) sessions: Vec<Session>,
    /// `None` shows the empty "new chat" state.
    pub(crate) active: Option<String>,
    pub(crate) running: Option<Running>,
    pub(crate) input: String,
    pub(crate) free_models: Vec<FreeModel>,
    free_rx: Option<Receiver<Vec<FreeModel>>>,
    pub(crate) md_cache: CommonMarkCache,
    /// Error shown above the composer (missing key, failed save, …).
    pub(crate) error: Option<String>,
    applied_palette: Option<Palette>,
}

impl BeamApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = channel();
        let ctx = cc.egui_ctx.clone();
        thread::spawn(move || {
            let _ = tx.send(catalog::fetch_free_models());
            ctx.request_repaint();
        });

        let cfg = Config::load();
        let palette = cfg.theme.palette();

        Self {
            cfg,
            palette,
            page: Page::Chat,
            sessions: session::load_all(),
            active: None,
            running: None,
            input: String::new(),
            free_models: catalog::fallback_free_models(),
            free_rx: Some(rx),
            md_cache: CommonMarkCache::default(),
            error: None,
            applied_palette: None,
        }
    }

    pub(crate) fn set_theme_preset(&mut self, preset: ThemePreset) {
        if preset != ThemePreset::Custom {
            // Keep `custom` seeded with the most recently selected preset so
            // switching to Custom starts from a familiar palette.
            self.cfg.theme.custom = RawPalette::from_preset(preset);
        }
        self.cfg.theme.preset = preset;
        self.palette = self.cfg.theme.palette();
        self.save_settings();
    }

    pub(crate) fn refresh_theme(&mut self) {
        self.palette = self.cfg.theme.palette();
        self.save_settings();
    }

    pub(crate) fn busy(&self) -> bool {
        self.running.is_some()
    }

    pub(crate) fn active_index(&self) -> Option<usize> {
        let id = self.active.as_ref()?;
        self.sessions.iter().position(|s| &s.id == id)
    }

    pub(crate) fn session_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.iter_mut().find(|s| s.id == id)
    }

    pub(crate) fn save_settings(&mut self) {
        if let Err(e) = self.cfg.save() {
            self.error = Some(format!("Could not save settings: {e:#}"));
        }
    }

    pub(crate) fn provider_name(&self, id: &str) -> String {
        if id == "custom" {
            return self.cfg.custom.name.clone();
        }
        catalog::find(id)
            .map(|p| p.name.to_string())
            .unwrap_or_else(|| id.to_string())
    }

    /// The model to use for a popular provider: the saved choice or its default.
    pub(crate) fn model_for(&self, p: &catalog::Provider) -> String {
        self.cfg
            .models
            .get(p.id)
            .cloned()
            .unwrap_or_else(|| p.default_model.to_string())
    }

    pub(crate) fn select(&mut self, provider: &str, model: &str) {
        self.cfg.active = Selection {
            provider: provider.to_string(),
            model: model.to_string(),
        };
        self.save_settings();
    }

    fn endpoint(&self) -> Result<Endpoint, String> {
        let sel = &self.cfg.active;
        let model = sel.model.trim().to_string();
        if model.is_empty() {
            return Err("Choose a model first.".into());
        }
        if sel.provider == "custom" {
            let c = &self.cfg.custom;
            if c.base_url.trim().is_empty() {
                return Err("Set a base URL for your custom endpoint in Settings.".into());
            }
            return Ok(Endpoint {
                wire: Wire::OpenAi,
                base_url: c.base_url.trim().to_string(),
                api_key: c.api_key.trim().to_string(),
                model,
            });
        }
        let p = catalog::find(&sel.provider)
            .ok_or_else(|| format!("Unknown provider '{}'.", sel.provider))?;
        let key = self.cfg.keys.get(p.id).cloned().unwrap_or_default();
        if p.needs_key && key.trim().is_empty() {
            return Err(format!("Add a {} API key in Settings.", p.name));
        }
        Ok(Endpoint {
            wire: p.wire,
            base_url: p.base_url.to_string(),
            api_key: key.trim().to_string(),
            model,
        })
    }

    /// Pulls agent events and the free-model list into state. Runs once per frame.
    fn poll(&mut self) {
        if let Some(rx) = &self.free_rx
            && let Ok(models) = rx.try_recv()
        {
            self.free_models = models;
            self.free_rx = None;
        }

        let Some(run) = self.running.as_ref() else {
            return;
        };
        let id = run.session_id.clone();
        let mut incoming = Vec::new();
        while let Ok(ev) = run.events.try_recv() {
            incoming.push(ev);
        }

        for ev in incoming {
            match ev {
                Event::Text(t) => self.push_entry(&id, Entry::Assistant(t)),
                Event::ToolStart { name, summary } => self.push_entry(
                    &id,
                    Entry::Tool {
                        name,
                        summary,
                        output: None,
                    },
                ),
                Event::ToolDone {
                    name,
                    output,
                    is_error,
                } => {
                    if let Some(s) = self.session_mut(&id) {
                        for e in s.entries.iter_mut().rev() {
                            if let Entry::Tool {
                                name: n,
                                output: slot,
                                ..
                            } = e
                                && *n == name
                                && slot.is_none()
                            {
                                *slot = Some((output, is_error));
                                break;
                            }
                        }
                    }
                }
                Event::Approval { description, reply } => {
                    if let Some(run) = self.running.as_mut() {
                        run.approval = Some((description, reply));
                    }
                }
                Event::Error(e) => self.push_entry(&id, Entry::Notice(e)),
                Event::Done(history) => {
                    if let Some(s) = self.session_mut(&id) {
                        s.history = history;
                        s.touch();
                    }
                    self.save_session(&id);
                    self.running = None;
                }
            }
        }
    }

    fn push_entry(&mut self, id: &str, entry: Entry) {
        if let Some(s) = self.session_mut(id) {
            s.entries.push(entry);
        }
    }

    pub(crate) fn save_session(&mut self, id: &str) {
        self.sort_sessions();
        if let Some(s) = self.sessions.iter().find(|s| s.id == id)
            && let Err(e) = session::save(s)
        {
            eprintln!("could not save session: {e:#}");
        }
    }

    fn sort_sessions(&mut self) {
        self.sessions.sort_by(|a, b| b.updated.cmp(&a.updated));
    }

    /// Starts an agent run for `text`. Returns false (and sets `error`) if it could not start.
    pub(crate) fn send(&mut self, text: String) -> bool {
        if self.busy() {
            return false;
        }
        let ep = match self.endpoint() {
            Ok(ep) => ep,
            Err(msg) => {
                self.error = Some(msg);
                return false;
            }
        };
        self.error = None;

        // A new chat is created on the first message.
        let id = match self.active.clone() {
            Some(id) => id,
            None => {
                let s = Session::new(self.cfg.workspace.clone());
                let id = s.id.clone();
                self.sessions.insert(0, s);
                self.active = Some(id.clone());
                id
            }
        };
        let auto_approve = self.cfg.auto_approve;
        let Some(s) = self.session_mut(&id) else {
            return false;
        };
        s.entries.push(Entry::User(text.clone()));
        s.name_from(&text);
        s.history.push(Msg::User(text));
        s.touch();
        let job = Job {
            ep,
            workspace: PathBuf::from(&s.workspace),
            auto_approve,
            history: std::mem::take(&mut s.history),
        };
        self.save_session(&id);

        let (tx, rx) = channel();
        self.running = Some(Running {
            session_id: id,
            events: rx,
            approval: None,
        });
        thread::spawn(move || agent::run(job, tx));
        true
    }

    pub(crate) fn resolve_approval(&mut self, allow: bool) {
        if let Some(run) = self.running.as_mut()
            && let Some((_, reply)) = run.approval.take()
        {
            let _ = reply.send(allow);
        }
    }

    pub(crate) fn new_chat(&mut self) {
        self.active = None;
        self.page = Page::Chat;
    }

    pub(crate) fn open_session(&mut self, id: String) {
        self.active = Some(id);
        self.page = Page::Chat;
    }

    pub(crate) fn delete_session(&mut self, id: &str) {
        if self.running.as_ref().is_some_and(|r| r.session_id == id) {
            return;
        }
        self.sessions.retain(|s| s.id != id);
        if self.active.as_deref() == Some(id) {
            self.active = None;
        }
        session::delete(id);
    }

    fn shortcuts(&mut self, ctx: &egui::Context) {
        let new_chat = KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::N);
        let settings = KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::Comma);
        let (new, open_settings, escape) = ctx.input_mut(|i| {
            (
                i.consume_shortcut(&new_chat),
                i.consume_shortcut(&settings),
                i.consume_key(Modifiers::NONE, egui::Key::Escape),
            )
        });
        if new && !self.busy() {
            self.new_chat();
        }
        if open_settings {
            self.page = if self.page == Page::Settings {
                Page::Chat
            } else {
                Page::Settings
            };
        }
        if escape && self.page == Page::Settings {
            self.page = Page::Chat;
        }
    }
}

impl eframe::App for BeamApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.applied_palette != Some(self.palette) {
            theme::apply(ui.ctx(), &self.palette);
            self.applied_palette = Some(self.palette);
        }
        self.poll();
        self.shortcuts(ui.ctx());
        let ctx = ui.ctx().clone();
        let p = self.palette;

        egui::Panel::left("sidebar")
            .default_size(232.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(p.sidebar)
                    .inner_margin(egui::Margin::symmetric(12, 12)),
            )
            .show(ui, |ui| views::sidebar::show(self, ui));

        if self.page == Page::Chat {
            egui::Panel::top("header")
                .frame(
                    egui::Frame::new()
                        .fill(p.bg)
                        .inner_margin(egui::Margin::symmetric(20, 10)),
                )
                .show(ui, |ui| views::chat::header(self, ui));
            egui::Panel::bottom("composer")
                .frame(
                    egui::Frame::new()
                        .fill(p.bg)
                        .inner_margin(egui::Margin::symmetric(20, 10)),
                )
                .show(ui, |ui| views::chat::composer(self, ui));
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(p.bg).inner_margin(egui::Margin::same(0)))
            .show(ui, |ui| match self.page {
                Page::Chat => views::chat::transcript(self, ui),
                Page::Settings => views::settings::show(self, ui),
            });

        if self.busy() || self.free_rx.is_some() {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}
