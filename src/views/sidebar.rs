//! Left rail: brand header, new chat button, session list, quick theme picker, and settings.

use crate::app::{BeamApp, Page};
use crate::theme::{RawPalette, ThemePreset};
use crate::tools::clip;
use eframe::egui::{
    self, Align, Button, Color32, CornerRadius, Frame, Label, Layout, Margin, RichText, ScrollArea,
    Stroke, Ui, vec2,
};

pub fn show(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;

    // Brand header
    ui.horizontal(|ui| {
        ui.label(RichText::new("◆").size(18.0).color(p.accent));
        ui.label(RichText::new("BeamNow").strong().size(19.0).color(p.text));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            Frame::new()
                .fill(p.accent_soft)
                .stroke(Stroke::new(1.0, p.accent.gamma_multiply(0.45)))
                .corner_radius(CornerRadius::same(20))
                .inner_margin(Margin::symmetric(8, 2))
                .show(ui, |ui| {
                    ui.label(RichText::new("v0.1").small().color(p.accent));
                });
        });
    });

    ui.add_space(12.0);

    // Primary "+ New chat" button
    let new_btn = Button::new(
        RichText::new("+  New Chat")
            .strong()
            .color(p.on_accent()),
    )
    .fill(p.accent)
    .stroke(Stroke::NONE)
    .corner_radius(CornerRadius::same(p.corner_radius))
    .min_size(vec2(ui.available_width(), 36.0));

    if ui
        .add_enabled(!app.busy(), new_btn)
        .on_hover_text("Start a fresh conversation (Ctrl+N)")
        .clicked()
    {
        app.new_chat();
    }

    ui.add_space(16.0);

    // Sessions header with counter
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("SESSIONS")
                .size(11.0)
                .strong()
                .color(p.muted),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if !app.sessions.is_empty() {
                ui.label(
                    RichText::new(format!("{}", app.sessions.len()))
                        .small()
                        .color(p.muted),
                );
            }
        });
    });
    ui.add_space(4.0);

    let footer_reserve = 96.0;
    let list_height = (ui.available_height() - footer_reserve).max(60.0);
    let mut open: Option<String> = None;
    let mut delete: Option<String> = None;

    ScrollArea::vertical()
        .id_salt("sidebar_sessions")
        .max_height(list_height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if app.sessions.is_empty() {
                Frame::new()
                    .fill(p.bg.gamma_multiply(0.5))
                    .stroke(Stroke::new(1.0, p.border))
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(12, 14))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.label(RichText::new("No chats yet").strong().color(p.text));
                        ui.add_space(2.0);
                        ui.label(
                            RichText::new("Send a message or pick a starter prompt to begin.")
                                .small()
                                .color(p.muted),
                        );
                    });
            }

            for s in &app.sessions {
                let selected = app.active.as_deref() == Some(s.id.as_str()) && app.page == Page::Chat;
                let running = app.running.as_ref().is_some_and(|r| r.session_id == s.id);
                let msg_count = s.entries.len();

                let fill = if selected {
                    p.selected
                } else {
                    Color32::TRANSPARENT
                };
                let stroke = if selected {
                    Stroke::new(1.0, p.accent.gamma_multiply(0.75))
                } else {
                    Stroke::new(1.0, Color32::TRANSPARENT)
                };

                let frame_resp = Frame::new()
                    .fill(fill)
                    .stroke(stroke)
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(10, 8))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            if running {
                                ui.label(RichText::new("●").color(p.ok));
                            } else if selected {
                                ui.label(RichText::new("▸").color(p.accent));
                            } else {
                                ui.label(RichText::new("·").color(p.muted));
                            }

                            let title_color = if selected { p.text } else { p.text.gamma_multiply(0.9) };
                            let title_resp = ui.add(
                                Label::new(
                                    RichText::new(clip(&s.title, 26))
                                        .strong()
                                        .color(title_color),
                                )
                                .truncate()
                                .selectable(false),
                            );

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if !running {
                                    let del_btn = Button::new(
                                        RichText::new("×").size(14.0).color(p.muted),
                                    )
                                    .frame(false)
                                    .min_size(vec2(18.0, 18.0));
                                    if ui
                                        .add(del_btn)
                                        .on_hover_text("Delete session")
                                        .clicked()
                                    {
                                        delete = Some(s.id.clone());
                                    }
                                }
                                ui.label(
                                    RichText::new(format!("{msg_count}"))
                                        .size(11.0)
                                        .color(p.muted),
                                );
                            });

                            title_resp
                        })
                        .inner
                    });

                let row_resp = frame_resp.response.interact(egui::Sense::click());
                if row_resp.clicked() && delete.as_deref() != Some(s.id.as_str()) {
                    open = Some(s.id.clone());
                }
                row_resp.context_menu(|ui| {
                    if ui
                        .add_enabled(!running, Button::new("Delete session"))
                        .clicked()
                    {
                        delete = Some(s.id.clone());
                        ui.close();
                    }
                });

                ui.add_space(2.0);
            }
        });

    if let Some(id) = open {
        app.open_session(id);
    }
    if let Some(id) = delete {
        app.delete_session(&id);
    }

    // Bottom footer: Theme quick-switcher + Settings button
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        let settings_open = app.page == Page::Settings;
        let settings_fill = if settings_open { p.selected } else { p.input };
        let settings_stroke = if settings_open {
            Stroke::new(1.0, p.accent)
        } else {
            Stroke::new(1.0, p.border)
        };
        let settings_btn = Button::new(
            RichText::new(if settings_open {
                "←  Back to Chat      Esc"
            } else {
                "⚙  Settings & Themes   Ctrl+,"
            })
            .color(p.text),
        )
        .fill(settings_fill)
        .stroke(settings_stroke)
        .corner_radius(CornerRadius::same(p.corner_radius))
        .min_size(vec2(ui.available_width(), 34.0));

        if ui
            .add(settings_btn)
            .on_hover_text("Configure providers, keys, and custom themes")
            .clicked()
        {
            app.page = if settings_open {
                Page::Chat
            } else {
                Page::Settings
            };
        }

        ui.add_space(6.0);

        // Quick Theme Preset Bar
        ui.horizontal(|ui| {
            ui.label(RichText::new("Theme").small().color(p.muted));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let current_preset = app.cfg.theme.preset;
                let mut picked: Option<ThemePreset> = None;
                egui::ComboBox::from_id_salt("sidebar_theme_preset")
                    .selected_text(format!("● {}", current_preset.name()))
                    .width(150.0)
                    .show_ui(ui, |ui| {
                        for &preset in ThemePreset::ALL {
                            let sample = if preset == ThemePreset::Custom {
                                app.cfg.theme.custom.resolve(app.cfg.theme.corner_radius)
                            } else {
                                RawPalette::from_preset(preset).resolve(app.cfg.theme.corner_radius)
                            };
                            ui.horizontal(|ui| {
                                ui.colored_label(sample.accent, "●");
                                if ui
                                    .selectable_label(
                                        current_preset == preset,
                                        format!("{} — {}", preset.name(), preset.subtitle()),
                                    )
                                    .clicked()
                                {
                                    picked = Some(preset);
                                }
                            });
                        }
                    });
                if let Some(preset) = picked {
                    app.set_theme_preset(preset);
                }
            });
        });

        ui.add_space(6.0);
        ui.separator();
    });
}
