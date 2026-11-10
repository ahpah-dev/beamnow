//! Minimalist left rail: brand wordmark, new chat action, session list, and footer bar.

use crate::app::{BeamApp, Page};
use crate::theme::{RawPalette, ThemePreset};
use crate::tools::clip;
use eframe::egui::{self, Align, Button, CornerRadius, Layout, RichText, ScrollArea, Stroke, Ui, vec2};

pub fn show(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;

    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("BeamNow").strong().size(16.5).color(p.text));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let new_btn = Button::new(RichText::new("+ New").small().color(p.text))
                .fill(p.input)
                .stroke(Stroke::new(1.0, p.border))
                .corner_radius(CornerRadius::same(p.corner_radius));
            if ui
                .add_enabled(!app.busy(), new_btn)
                .on_hover_text("New chat (Ctrl+N)")
                .clicked()
            {
                app.new_chat();
            }
        });
    });

    ui.add_space(12.0);

    let footer_height = 40.0;
    let list_height = (ui.available_height() - footer_height).max(40.0);
    let mut open: Option<String> = None;
    let mut delete: Option<String> = None;

    ScrollArea::vertical()
        .id_salt("sidebar_sessions")
        .max_height(list_height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if app.sessions.is_empty() {
                ui.add_space(4.0);
                ui.label(RichText::new("No chats yet").small().color(p.muted));
            }

            for s in &app.sessions {
                let selected =
                    app.active.as_deref() == Some(s.id.as_str()) && app.page == Page::Chat;
                let running = app.running.as_ref().is_some_and(|r| r.session_id == s.id);
                let label = if running {
                    format!("●  {}", clip(&s.title, 26))
                } else {
                    clip(&s.title, 28)
                };

                let text_col = if selected { p.text } else { p.muted };
                let btn = Button::selectable(
                    selected,
                    RichText::new(label).color(text_col),
                )
                .min_size(vec2(ui.available_width(), 28.0));

                let resp = ui.add(btn);
                if resp.clicked() {
                    open = Some(s.id.clone());
                }
                resp.context_menu(|ui| {
                    if ui
                        .add_enabled(!running, Button::new("Delete"))
                        .clicked()
                    {
                        delete = Some(s.id.clone());
                        ui.close();
                    }
                });
            }
        });

    if let Some(id) = open {
        app.open_session(id);
    }
    if let Some(id) = delete {
        app.delete_session(&id);
    }

    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        ui.horizontal(|ui| {
            let settings_open = app.page == Page::Settings;
            let settings_resp = ui
                .add(Button::selectable(
                    settings_open,
                    RichText::new("Settings").small().color(if settings_open {
                        p.text
                    } else {
                        p.muted
                    }),
                ))
                .on_hover_text("Settings & themes (Ctrl+,)");
            if settings_resp.clicked() {
                app.page = if settings_open {
                    Page::Chat
                } else {
                    Page::Settings
                };
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let current = app.cfg.theme.preset;
                let mut picked: Option<ThemePreset> = None;
                egui::ComboBox::from_id_salt("quick_theme")
                    .selected_text(
                        RichText::new(format!("● {}", current.name()))
                            .small()
                            .color(p.muted),
                    )
                    .width(105.0)
                    .show_ui(ui, |ui| {
                        for &preset in ThemePreset::ALL {
                            let sample = if preset == ThemePreset::Custom {
                                app.cfg
                                    .theme
                                    .custom
                                    .resolve(app.cfg.theme.corner_radius)
                            } else {
                                RawPalette::from_preset(preset)
                                    .resolve(app.cfg.theme.corner_radius)
                            };
                            ui.horizontal(|ui| {
                                ui.colored_label(sample.accent, "●");
                                if ui
                                    .selectable_label(current == preset, preset.name())
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
    });
}
