//! Chat page: header, empty-state hero with starter prompts, transcript, and composer.

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

const STARTERS: &[(&str, &str, &str)] = &[
    (
        "Explore codebase",
        "Summarize the architecture and key modules",
        "Give me a concise architectural overview of this project and how its modules fit together.",
    ),
    (
        "Find bugs & risks",
        "Audit the workspace for edge cases",
        "Inspect the codebase for potential bugs, error-handling gaps, or edge cases and suggest fixes.",
    ),
    (
        "Add unit tests",
        "Write focused tests for core logic",
        "Find core functions in this project that would benefit from unit tests and add them.",
    ),
    (
        "Refactor & clean up",
        "Improve readability and idioms",
        "Review the code in this workspace and suggest clean, idiomatic improvements.",
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
    let active_model = format!(
        "{} · {}",
        app.provider_name(&app.cfg.active.provider),
        app.cfg.active.model
    );

    ui.horizontal(|ui| {
        ui.add(
            Label::new(RichText::new(title).strong().size(16.0).color(p.text)).truncate(),
        );

        Frame::new()
            .fill(p.tool_bg)
            .stroke(Stroke::new(1.0, p.border))
            .corner_radius(CornerRadius::same(20))
            .inner_margin(Margin::symmetric(8, 2))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(clip(&active_model, 36))
                        .small()
                        .color(p.muted),
                );
            });

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            Frame::new()
                .fill(p.tool_bg)
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(Margin::symmetric(8, 3))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(clip(&workspace, 44))
                            .small()
                            .monospace()
                            .color(p.muted),
                    );
                });

            if app.busy() {
                Frame::new()
                    .fill(p.accent_soft)
                    .stroke(Stroke::new(1.0, p.accent))
                    .corner_radius(CornerRadius::same(20))
                    .inner_margin(Margin::symmetric(8, 2))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(RichText::new("Running").small().strong().color(p.accent));
                        });
                    });
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
                ui.add_space(18.0);
                for (i, entry) in entries.iter().enumerate() {
                    render_entry(ui, i, entry, cache, &p);
                }
                if let Some(desc) = &approval {
                    ui.add_space(6.0);
                    decision = approval_card(ui, desc, &p);
                }
                if working {
                    ui.add_space(8.0);
                    Frame::new()
                        .fill(p.tool_bg)
                        .stroke(Stroke::new(1.0, p.border))
                        .corner_radius(CornerRadius::same(p.corner_radius))
                        .inner_margin(Margin::symmetric(12, 8))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label(
                                    RichText::new("BeamNow is thinking…")
                                        .small()
                                        .color(p.muted),
                                );
                            });
                        });
                }
                ui.add_space(20.0);
            });
        });

    if let Some(allow) = decision {
        app.resolve_approval(allow);
    }
}

fn empty_state(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let mut chosen_prompt: Option<String> = None;

    ScrollArea::vertical()
        .id_salt("empty_state_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let top_pad = (ui.available_height() * 0.14).max(24.0);
            ui.add_space(top_pad);

            column(ui, 660.0, |ui| {
                ui.vertical_centered(|ui| {
                    Frame::new()
                        .fill(p.accent_soft)
                        .stroke(Stroke::new(1.0, p.accent))
                        .corner_radius(CornerRadius::same(24))
                        .inner_margin(Margin::symmetric(14, 6))
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("◆  BeamNow Coding Agent")
                                    .small()
                                    .strong()
                                    .color(p.accent),
                            );
                        });

                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("What should we build today?")
                            .size(26.0)
                            .strong()
                            .color(p.text),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(&app.cfg.workspace)
                            .small()
                            .monospace()
                            .color(p.muted),
                    );
                });

                ui.add_space(24.0);

                // 2x2 starter prompt cards
                let card_w = ((ui.available_width() - 12.0) / 2.0).max(180.0);
                for pair in STARTERS.chunks(2) {
                    ui.horizontal(|ui| {
                        for &(title, subtitle, prompt) in pair {
                            let resp = Frame::new()
                                .fill(p.tool_bg)
                                .stroke(Stroke::new(1.0, p.border))
                                .corner_radius(CornerRadius::same(p.corner_radius))
                                .inner_margin(Margin::symmetric(14, 12))
                                .show(ui, |ui| {
                                    ui.set_width(card_w - 28.0);
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(title).strong().color(p.text),
                                        );
                                        ui.with_layout(
                                            Layout::right_to_left(Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new("→").color(p.accent),
                                                );
                                            },
                                        );
                                    });
                                    ui.add_space(2.0);
                                    ui.label(
                                        RichText::new(subtitle).small().color(p.muted),
                                    );
                                })
                                .response
                                .interact(egui::Sense::click());

                            if resp
                                .on_hover_text("Click to use this prompt")
                                .clicked()
                            {
                                chosen_prompt = Some(prompt.to_string());
                            }
                            ui.add_space(4.0);
                        }
                    });
                    ui.add_space(4.0);
                }
            });
        });

    if let Some(prompt) = chosen_prompt {
        app.input = prompt;
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
            Frame::new()
                .fill(p.user_bubble)
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(CornerRadius::same(p.corner_radius))
                .inner_margin(Margin::symmetric(16, 12))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("YOU")
                                .size(11.0)
                                .strong()
                                .color(p.accent),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add(
                                    Button::new(RichText::new("Copy").small().color(p.muted))
                                        .frame(false),
                                )
                                .on_hover_text("Copy prompt")
                                .clicked()
                            {
                                ui.ctx().copy_text(text.clone());
                            }
                        });
                    });
                    ui.add_space(4.0);
                    ui.label(RichText::new(text.as_str()).color(p.text));
                });
            ui.add_space(6.0);
        }
        Entry::Assistant(text) => {
            ui.add_space(8.0);
            Frame::new()
                .fill(p.bg)
                .stroke(Stroke::new(1.0, p.border.gamma_multiply(0.65)))
                .corner_radius(CornerRadius::same(p.corner_radius))
                .inner_margin(Margin::symmetric(16, 12))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("◆ BEAMNOW")
                                .size(11.0)
                                .strong()
                                .color(p.accent),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add(
                                    Button::new(RichText::new("Copy").small().color(p.muted))
                                        .frame(false),
                                )
                                .on_hover_text("Copy response markdown")
                                .clicked()
                            {
                                ui.ctx().copy_text(text.clone());
                            }
                        });
                    });
                    ui.add_space(4.0);
                    CommonMarkViewer::new().show(ui, cache, text);
                });
            ui.add_space(6.0);
        }
        Entry::Tool {
            name,
            summary,
            output,
        } => {
            ui.add_space(3.0);
            let border_col = match output {
                None => p.accent.gamma_multiply(0.6),
                Some((_, true)) => p.err.gamma_multiply(0.6),
                Some((_, false)) => p.border,
            };
            Frame::new()
                .fill(p.tool_bg)
                .stroke(Stroke::new(1.0, border_col))
                .corner_radius(CornerRadius::same(p.corner_radius.saturating_sub(2)))
                .inner_margin(Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        match output {
                            None => {
                                ui.spinner();
                            }
                            Some((_, true)) => {
                                ui.colored_label(p.err, "✖");
                            }
                            Some(_) => {
                                ui.colored_label(p.ok, "✔");
                            }
                        }
                        Frame::new()
                            .fill(p.input)
                            .stroke(Stroke::new(1.0, p.border))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(Margin::symmetric(6, 2))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(name)
                                        .small()
                                        .monospace()
                                        .strong()
                                        .color(p.text),
                                );
                            });
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
                        ui.add_space(2.0);
                        egui::CollapsingHeader::new(
                            RichText::new(format!("Output ({} chars)", out.len()))
                                .small()
                                .color(p.muted),
                        )
                        .id_salt(("tool-output", i))
                        .default_open(false)
                        .show(ui, |ui| {
                            Frame::new()
                                .fill(p.bg)
                                .stroke(Stroke::new(1.0, p.border))
                                .corner_radius(CornerRadius::same(6))
                                .inner_margin(Margin::same(8))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ScrollArea::vertical()
                                        .id_salt(("tool-output-scroll", i))
                                        .max_height(240.0)
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(clip(out, 6000))
                                                    .small()
                                                    .monospace()
                                                    .color(p.text),
                                            );
                                        });
                                });
                        });
                    }
                });
            ui.add_space(3.0);
        }
        Entry::Notice(text) => {
            ui.add_space(6.0);
            Frame::new()
                .fill(p.err.gamma_multiply(0.10))
                .stroke(Stroke::new(1.0, p.err))
                .corner_radius(CornerRadius::same(p.corner_radius))
                .inner_margin(Margin::symmetric(14, 10))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.colored_label(p.err, "⚠");
                        ui.colored_label(p.err, text.as_str());
                    });
                });
            ui.add_space(6.0);
        }
    }
}

/// Inline permission request. Returns the user's choice once clicked.
fn approval_card(ui: &mut Ui, desc: &str, p: &Palette) -> Option<bool> {
    let mut out = None;
    Frame::new()
        .fill(p.accent_soft)
        .stroke(Stroke::new(1.5, p.accent))
        .corner_radius(CornerRadius::same(p.corner_radius))
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new("⚡ Permission Needed")
                    .strong()
                    .color(p.accent),
            );
            ui.add_space(4.0);
            Frame::new()
                .fill(p.bg)
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(CornerRadius::same(6))
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                        ui.label(RichText::new(desc).monospace().small().color(p.text));
                    });
                });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let allow = Button::new(
                    RichText::new("✔  Allow")
                        .strong()
                        .color(p.on_accent()),
                )
                .fill(p.accent)
                .min_size(vec2(92.0, 30.0));
                if ui.add(allow).clicked() {
                    out = Some(true);
                }
                let deny = Button::new(RichText::new("✖  Deny").color(p.text))
                    .fill(p.input)
                    .stroke(Stroke::new(1.0, p.border))
                    .min_size(vec2(84.0, 30.0));
                if ui.add(deny).clicked() {
                    out = Some(false);
                }
            });
        });
    out
}

pub fn composer(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    ui.add_space(6.0);
    column(ui, theme::COLUMN, |ui| {
        if let Some(err) = app.error.clone() {
            Frame::new()
                .fill(p.err.gamma_multiply(0.12))
                .stroke(Stroke::new(1.0, p.err))
                .corner_radius(CornerRadius::same(p.corner_radius))
                .inner_margin(Margin::symmetric(12, 8))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(p.err, format!("⚠  {err}"));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.small_button("Dismiss").clicked() {
                                app.error = None;
                            }
                        });
                    });
                });
            ui.add_space(6.0);
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
                        .hint_text("Ask BeamNow to write code, inspect files, or run commands…"),
                );

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    model_menu(app, ui);
                    permission_menu(app, ui);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let can_send = !busy && !app.input.trim().is_empty();
                        let send_fill = if can_send { p.accent } else { p.tool_bg };
                        let send_fg = if can_send { p.on_accent() } else { p.muted };
                        let send_btn = Button::new(
                            RichText::new("Send  ↵").strong().color(send_fg),
                        )
                        .fill(send_fill)
                        .stroke(Stroke::new(
                            1.0,
                            if can_send { p.accent } else { p.border },
                        ))
                        .min_size(vec2(84.0, 28.0));

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

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Enter to send · Shift+Enter new line · Ctrl+N new chat · Ctrl+, settings")
                    .small()
                    .color(p.muted),
            );
        });
    });
    ui.add_space(6.0);
}

fn model_menu(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let current = format!(
        "◆ {} · {}",
        app.provider_name(&app.cfg.active.provider),
        app.cfg.active.model
    );
    let mut choice: Option<(String, String)> = None;
    egui::ComboBox::from_id_salt("model")
        .selected_text(clip(&current, 42))
        .width(230.0)
        .show_ui(ui, |ui| {
            ui.label(RichText::new("Free Models").small().strong().color(p.muted));
            for m in &app.free_models {
                let label = format!("{} · {}", app.provider_name(m.provider), m.id);
                let selected =
                    app.cfg.active.provider == m.provider && app.cfg.active.model == m.id;
                if ui.selectable_label(selected, label).clicked() {
                    choice = Some((m.provider.to_string(), m.id.clone()));
                }
            }
            ui.separator();
            ui.label(RichText::new("Configured Providers").small().strong().color(p.muted));
            for prov in crate::catalog::POPULAR {
                let has_key = !prov.needs_key
                    || app
                        .cfg
                        .keys
                        .get(prov.id)
                        .is_some_and(|k| !k.trim().is_empty());
                if has_key {
                    let model = app.model_for(prov);
                    let selected = app.cfg.active.provider == prov.id && app.cfg.active.model == model;
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
    let label = if app.cfg.auto_approve {
        "⚡ Auto-approve"
    } else {
        "🛡 Ask before changes"
    };
    let mut changed = false;
    egui::ComboBox::from_id_salt("permissions")
        .selected_text(label)
        .width(165.0)
        .show_ui(ui, |ui| {
            changed |= ui
                .selectable_value(&mut app.cfg.auto_approve, false, "🛡 Ask before changes")
                .changed();
            changed |= ui
                .selectable_value(&mut app.cfg.auto_approve, true, "⚡ Auto-approve all")
                .changed();
        });
    if changed {
        app.save_settings();
    }
}
