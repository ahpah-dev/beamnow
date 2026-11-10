//! Minimalist chat page: header, calm empty state, clean transcript, and floating composer.

use super::column;
use crate::app::BeamApp;
use crate::session::Entry;
use crate::theme::{self, Palette};
use crate::tools::clip;
use eframe::egui::{
    self, Align, Button, CornerRadius, Frame, Label, Layout, Margin, Modifiers, RichText,
    ScrollArea, Stroke, TextEdit, Ui, vec2,
};
use egui_commonmark::CommonMarkViewer;

const PROMPTS: &[(&str, &str)] = &[
    (
        "Explain codebase",
        "Give me a concise overview of this repository's structure and key modules.",
    ),
    (
        "Find bugs",
        "Inspect the code in this workspace for potential bugs or edge cases.",
    ),
    (
        "Write tests",
        "Identify core functions lacking coverage and add clean unit tests.",
    ),
    (
        "Refactor",
        "Suggest and apply clean, idiomatic improvements to the codebase.",
    ),
];

pub fn header(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let (title, workspace) = match app.active_index() {
        Some(i) => (
            app.sessions[i].title.clone(),
            app.sessions[i].workspace.clone(),
        ),
        None => ("New chat".to_string(), app.cfg.workspace.clone()),
    };

    ui.horizontal(|ui| {
        ui.add(Label::new(RichText::new(title).strong().color(p.text)).truncate());
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add(
                Label::new(
                    RichText::new(clip(&workspace, 48))
                        .small()
                        .monospace()
                        .color(p.muted),
                )
                .truncate(),
            );
            if app.busy() {
                ui.spinner();
            }
        });
    });
}

pub fn transcript(app: &mut BeamApp, ui: &mut Ui) {
    let Some(idx) = app.active_index() else {
        empty_state(app, ui);
        return;
    };
    let p = app.palette;
    let id = app.sessions[idx].id.clone();
    let entries = &app.sessions[idx].entries;
    let cache = &mut app.md_cache;
    let running = app.running.as_ref().filter(|r| r.session_id == id);
    let approval = running
        .and_then(|r| r.approval.as_ref())
        .map(|(desc, _)| desc.clone());
    let working = running.is_some() && approval.is_none();

    let mut decision = None;
    ScrollArea::vertical()
        .id_salt("chat_transcript")
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            column(ui, theme::COLUMN, |ui| {
                ui.add_space(12.0);
                for (i, entry) in entries.iter().enumerate() {
                    render_entry(ui, i, entry, cache, &p);
                }
                if let Some(desc) = &approval {
                    ui.add_space(6.0);
                    decision = approval_card(ui, desc, &p);
                }
                if working {
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(RichText::new("Thinking…").small().color(p.muted));
                    });
                }
                ui.add_space(16.0);
            });
        });

    if let Some(allow) = decision {
        app.resolve_approval(allow);
    }
}

fn empty_state(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let space = (ui.available_height() * 0.26).max(32.0);
    let mut picked: Option<&str> = None;

    ui.vertical_centered(|ui| {
        ui.add_space(space);
        ui.label(
            RichText::new("What should we build?")
                .size(22.0)
                .strong()
                .color(p.text),
        );
        ui.add_space(2.0);
        ui.label(
            RichText::new(&app.cfg.workspace)
                .small()
                .monospace()
                .color(p.muted),
        );
        ui.add_space(18.0);

        column(ui, 520.0, |ui| {
            ui.horizontal_wrapped(|ui| {
                for &(label, prompt) in PROMPTS {
                    let btn = Button::new(RichText::new(label).small().color(p.muted))
                        .fill(p.tool_bg)
                        .stroke(Stroke::new(1.0, p.border))
                        .corner_radius(CornerRadius::same(20));
                    if ui.add(btn).clicked() {
                        picked = Some(prompt);
                    }
                }
            });
        });
    });

    if let Some(prompt) = picked {
        app.input = prompt.to_string();
    }
}

fn render_entry(
    ui: &mut Ui,
    i: usize,
    entry: &Entry,
    cache: &mut egui_commonmark::CommonMarkCache,
    p: &Palette,
) {
    match entry {
        Entry::User(text) => {
            ui.add_space(10.0);
            let resp = Frame::new()
                .fill(p.user_bubble)
                .corner_radius(CornerRadius::same(p.corner_radius.saturating_add(2)))
                .inner_margin(Margin::symmetric(14, 10))
                .show(ui, |ui| {
                    ui.label(RichText::new(text.as_str()).color(p.text));
                })
                .response;
            resp.context_menu(|ui| {
                if ui.button("Copy").clicked() {
                    ui.ctx().copy_text(text.clone());
                    ui.close();
                }
            });
            ui.add_space(4.0);
        }
        Entry::Assistant(text) => {
            ui.add_space(6.0);
            CommonMarkViewer::new().show(ui, cache, text);
            ui.add_space(4.0);
        }
        Entry::Tool {
            name,
            summary,
            output,
        } => {
            ui.add_space(2.0);
            Frame::new()
                .fill(p.tool_bg)
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(CornerRadius::same(p.corner_radius))
                .inner_margin(Margin::symmetric(12, 7))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        match output {
                            None => {
                                ui.spinner();
                            }
                            Some((_, true)) => {
                                ui.colored_label(p.err, "✖");
                            }
                            Some(_) => {
                                ui.colored_label(p.ok, "✓");
                            }
                        }
                        ui.label(RichText::new(name).small().monospace().strong().color(p.text));
                        ui.add(
                            Label::new(
                                RichText::new(summary)
                                    .small()
                                    .monospace()
                                    .color(p.muted),
                            )
                            .truncate(),
                        );
                    });
                    if let Some((out, _)) = output
                        && !out.trim().is_empty()
                    {
                        egui::CollapsingHeader::new(
                            RichText::new("Output").small().color(p.muted),
                        )
                        .id_salt(("tool-output", i))
                        .default_open(false)
                        .show(ui, |ui| {
                            ScrollArea::vertical()
                                .id_salt(("tool-output-scroll", i))
                                .max_height(220.0)
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(clip(out, 6000))
                                            .small()
                                            .monospace()
                                            .color(p.text),
                                    );
                                });
                        });
                    }
                });
            ui.add_space(2.0);
        }
        Entry::Notice(text) => {
            ui.add_space(4.0);
            Frame::new()
                .stroke(Stroke::new(1.0, p.err))
                .corner_radius(CornerRadius::same(p.corner_radius))
                .inner_margin(Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.colored_label(p.err, text.as_str());
                });
            ui.add_space(4.0);
        }
    }
}

/// Inline permission request. Returns the user's choice once clicked.
fn approval_card(ui: &mut Ui, desc: &str, p: &Palette) -> Option<bool> {
    let mut out = None;
    Frame::new()
        .fill(p.accent_soft)
        .stroke(Stroke::new(1.0, p.accent))
        .corner_radius(CornerRadius::same(p.corner_radius))
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            ui.label(RichText::new("Permission needed").strong().color(p.accent));
            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                ui.label(RichText::new(desc).small().monospace().color(p.text));
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let allow = Button::new(RichText::new("Allow").color(p.on_accent()))
                    .fill(p.accent);
                if ui.add(allow).clicked() {
                    out = Some(true);
                }
                if ui.button("Deny").clicked() {
                    out = Some(false);
                }
            });
        });
    out
}

pub fn composer(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    column(ui, theme::COLUMN, |ui| {
        if let Some(err) = app.error.clone() {
            ui.horizontal(|ui| {
                ui.colored_label(p.err, err);
                if ui.small_button("dismiss").clicked() {
                    app.error = None;
                }
            });
        }

        // Plain Enter sends; Shift+Enter adds a line. Consumed before the editor sees it.
        let enter = ui.input_mut(|i| i.consume_key(Modifiers::NONE, egui::Key::Enter));
        let busy = app.busy();

        Frame::new()
            .fill(p.input)
            .stroke(Stroke::new(1.0, p.border))
            .corner_radius(CornerRadius::same(p.corner_radius.saturating_add(4)))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                let edit = ui.add_enabled(
                    !busy,
                    TextEdit::multiline(&mut app.input)
                        .frame(Frame::NONE)
                        .desired_rows(2)
                        .desired_width(f32::INFINITY)
                        .hint_text("Message BeamNow…"),
                );

                ui.horizontal(|ui| {
                    model_menu(app, ui);
                    permission_menu(app, ui);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let can_send = !busy && !app.input.trim().is_empty();
                        let send_btn = if can_send {
                            Button::new(RichText::new("Send").strong().color(p.on_accent()))
                                .fill(p.accent)
                                .min_size(vec2(64.0, 26.0))
                        } else {
                            Button::new(RichText::new("Send").color(p.muted))
                                .min_size(vec2(64.0, 26.0))
                        };
                        let clicked = ui.add_enabled(can_send, send_btn).clicked();
                        if clicked || (enter && edit.has_focus() && can_send) {
                            let text = app.input.trim().to_string();
                            if app.send(text) {
                                app.input.clear();
                            }
                        }
                    });
                });
            });
    });
}

fn model_menu(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let current = format!(
        "{} · {}",
        app.provider_name(&app.cfg.active.provider),
        app.cfg.active.model
    );
    let mut choice: Option<(String, String)> = None;
    egui::ComboBox::from_id_salt("model")
        .selected_text(RichText::new(clip(&current, 38)).small().color(p.muted))
        .width(210.0)
        .show_ui(ui, |ui| {
            ui.label(RichText::new("Free").small().color(p.muted));
            for m in &app.free_models {
                let label = format!("{} · {}", app.provider_name(m.provider), m.id);
                let selected =
                    app.cfg.active.provider == m.provider && app.cfg.active.model == m.id;
                if ui.selectable_label(selected, label).clicked() {
                    choice = Some((m.provider.to_string(), m.id.clone()));
                }
            }
            ui.separator();
            ui.label(RichText::new("Your keys").small().color(p.muted));
            for prov in crate::catalog::POPULAR {
                let has_key = !prov.needs_key
                    || app
                        .cfg
                        .keys
                        .get(prov.id)
                        .is_some_and(|k| !k.trim().is_empty());
                if has_key {
                    let model = app.model_for(prov);
                    let selected =
                        app.cfg.active.provider == prov.id && app.cfg.active.model == model;
                    if ui
                        .selectable_label(selected, format!("{} · {}", prov.name, model))
                        .clicked()
                    {
                        choice = Some((prov.id.to_string(), model));
                    }
                }
            }
            if !app.cfg.custom.base_url.trim().is_empty()
                && !app.cfg.custom.model.trim().is_empty()
            {
                let selected = app.cfg.active.provider == "custom";
                let label = format!("{} · {}", app.cfg.custom.name, app.cfg.custom.model);
                if ui.selectable_label(selected, label).clicked() {
                    choice = Some(("custom".to_string(), app.cfg.custom.model.clone()));
                }
            }
        });
    if let Some((provider, model)) = choice {
        app.select(&provider, &model);
    }
}

fn permission_menu(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let label = if app.cfg.auto_approve {
        "Auto-approve"
    } else {
        "Ask before changes"
    };
    let mut changed = false;
    egui::ComboBox::from_id_salt("permissions")
        .selected_text(RichText::new(label).small().color(p.muted))
        .width(145.0)
        .show_ui(ui, |ui| {
            changed |= ui
                .selectable_value(&mut app.cfg.auto_approve, false, "Ask before changes")
                .changed();
            changed |= ui
                .selectable_value(&mut app.cfg.auto_approve, true, "Auto-approve")
                .changed();
        });
    if changed {
        app.save_settings();
    }
}
