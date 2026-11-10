//! Settings page: customizable themes, BYOK providers, custom endpoints, and workspace preferences.

use super::column;
use crate::app::{BeamApp, Page};
use crate::catalog;
use crate::theme::{Palette, RawPalette, ThemePreset};
use eframe::egui::{
    self, Align, Button, Color32, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea,
    Slider, Stroke, Ui, vec2,
};

pub fn show(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;

    // Top bar
    Frame::new()
        .fill(p.bg)
        .stroke(Stroke::new(1.0, p.border))
        .inner_margin(Margin::symmetric(18, 10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let back_btn = Button::new(RichText::new("←  Back to Chat").color(p.text))
                    .fill(p.input)
                    .stroke(Stroke::new(1.0, p.border))
                    .min_size(vec2(120.0, 30.0));
                if ui.add(back_btn).clicked() {
                    app.page = Page::Chat;
                }
                ui.add_space(6.0);
                ui.label(
                    RichText::new("Settings & Appearance")
                        .strong()
                        .size(18.0)
                        .color(p.text),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if let Some(path) = crate::config::Config::path() {
                        ui.label(
                            RichText::new(path.display().to_string())
                                .small()
                                .monospace()
                                .color(p.muted),
                        );
                    }
                });
            });
        });

    let mut use_choice: Option<(String, String)> = None;
    let mut preset_choice: Option<ThemePreset> = None;
    let mut theme_dirty = false;
    let mut dirty = false;

    ScrollArea::vertical()
        .id_salt("settings_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            column(ui, 760.0, |ui| {
                ui.add_space(18.0);

                // =========================================================
                // 1. APPEARANCE & CUSTOMIZABLE THEMES
                // =========================================================
                section_header(
                    ui,
                    &p,
                    "Appearance & Themes",
                    "Choose a curated preset or craft your own custom color palette.",
                );
                ui.add_space(8.0);

                // Preset Swatch Grid (4 per row)
                let card_w = ((ui.available_width() - 24.0) / 4.0).max(140.0);
                for row in ThemePreset::ALL.chunks(4) {
                    ui.horizontal(|ui| {
                        for &preset in row {
                            let is_active = app.cfg.theme.preset == preset;
                            let sample = if preset == ThemePreset::Custom {
                                app.cfg
                                    .theme
                                    .custom
                                    .resolve(app.cfg.theme.corner_radius)
                            } else {
                                RawPalette::from_preset(preset)
                                    .resolve(app.cfg.theme.corner_radius)
                            };

                            let stroke = if is_active {
                                Stroke::new(2.0, p.accent)
                            } else {
                                Stroke::new(1.0, p.border)
                            };
                            let fill = if is_active { p.selected } else { p.tool_bg };

                            let resp = Frame::new()
                                .fill(fill)
                                .stroke(stroke)
                                .corner_radius(CornerRadius::same(p.corner_radius))
                                .inner_margin(Margin::symmetric(12, 10))
                                .show(ui, |ui| {
                                    ui.set_width(card_w - 24.0);
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(preset.name())
                                                .strong()
                                                .color(p.text),
                                        );
                                        if is_active {
                                            ui.with_layout(
                                                Layout::right_to_left(Align::Center),
                                                |ui| {
                                                    ui.label(
                                                        RichText::new("●")
                                                            .small()
                                                            .color(p.accent),
                                                    );
                                                },
                                            );
                                        }
                                    });
                                    ui.label(
                                        RichText::new(preset.subtitle())
                                            .size(11.0)
                                            .color(p.muted),
                                    );
                                    ui.add_space(6.0);
                                    // Color swatch preview bar
                                    ui.horizontal(|ui| {
                                        swatch_dot(ui, sample.bg);
                                        swatch_dot(ui, sample.sidebar);
                                        swatch_dot(ui, sample.input);
                                        swatch_dot(ui, sample.accent);
                                        swatch_dot(ui, sample.text);
                                    });
                                })
                                .response
                                .interact(egui::Sense::click());

                            if resp.clicked() {
                                preset_choice = Some(preset);
                            }
                        }
                    });
                    ui.add_space(4.0);
                }

                ui.add_space(6.0);

                // Custom Theme Editor & Geometry Card
                Frame::new()
                    .fill(p.tool_bg)
                    .stroke(Stroke::new(1.0, p.border))
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(16, 14))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Customize Palette & Geometry")
                                    .strong()
                                    .color(p.text),
                            );
                            ui.label(
                                RichText::new(
                                    "Editing any color switches to the Custom preset automatically.",
                                )
                                .small()
                                .color(p.muted),
                            );
                        });
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Corner radius").color(p.text));
                            let mut radius = app.cfg.theme.corner_radius as u32;
                            if ui
                                .add(Slider::new(&mut radius, 2..=18).suffix(" px"))
                                .changed()
                            {
                                app.cfg.theme.corner_radius = radius as u8;
                                theme_dirty = true;
                            }

                            ui.separator();

                            if ui
                                .checkbox(
                                    &mut app.cfg.theme.custom.dark_mode,
                                    "Dark base visuals",
                                )
                                .changed()
                            {
                                app.cfg.theme.preset = ThemePreset::Custom;
                                theme_dirty = true;
                            }

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui
                                    .button("Reset Custom to Ember")
                                    .on_hover_text("Restore default Ember colors into Custom")
                                    .clicked()
                                {
                                    app.cfg.theme.custom =
                                        RawPalette::from_preset(ThemePreset::Ember);
                                    app.cfg.theme.preset = ThemePreset::Custom;
                                    theme_dirty = true;
                                }
                            });
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(6.0);

                        let custom = &mut app.cfg.theme.custom;
                        let mut color_edited = false;

                        ui.label(
                            RichText::new("SURFACES")
                                .size(11.0)
                                .strong()
                                .color(p.muted),
                        );
                        ui.horizontal_wrapped(|ui| {
                            color_edited |= color_field(ui, "Background", &mut custom.bg, &p);
                            color_edited |= color_field(ui, "Sidebar", &mut custom.sidebar, &p);
                            color_edited |= color_field(ui, "Input / Card", &mut custom.input, &p);
                            color_edited |=
                                color_field(ui, "User Bubble", &mut custom.user_bubble, &p);
                            color_edited |=
                                color_field(ui, "Tool Surface", &mut custom.tool_bg, &p);
                        });

                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("ACCENTS & INTERACTION")
                                .size(11.0)
                                .strong()
                                .color(p.muted),
                        );
                        ui.horizontal_wrapped(|ui| {
                            color_edited |= color_field(ui, "Accent", &mut custom.accent, &p);
                            color_edited |=
                                color_field(ui, "Soft Accent", &mut custom.accent_soft, &p);
                            color_edited |= color_field(ui, "Border", &mut custom.border, &p);
                            color_edited |= color_field(ui, "Hover", &mut custom.hover, &p);
                            color_edited |= color_field(ui, "Selected", &mut custom.selected, &p);
                        });

                        ui.add_space(6.0);
                        ui.label(
                            RichText::new("TYPOGRAPHY & STATUS")
                                .size(11.0)
                                .strong()
                                .color(p.muted),
                        );
                        ui.horizontal_wrapped(|ui| {
                            color_edited |= color_field(ui, "Primary Text", &mut custom.text, &p);
                            color_edited |= color_field(ui, "Muted Text", &mut custom.muted, &p);
                            color_edited |= color_field(ui, "Success (OK)", &mut custom.ok, &p);
                            color_edited |= color_field(ui, "Error", &mut custom.err, &p);
                        });

                        if color_edited {
                            app.cfg.theme.preset = ThemePreset::Custom;
                            theme_dirty = true;
                        }
                    });

                ui.add_space(22.0);

                // =========================================================
                // 2. MODEL PROVIDERS (POPULAR / BYOK)
                // =========================================================
                section_header(
                    ui,
                    &p,
                    "Model Providers",
                    "Bring your own API key. Keys are stored locally in your BeamNow config file.",
                );
                ui.add_space(8.0);

                let cfg = &mut app.cfg;
                for prov in catalog::POPULAR {
                    let active = cfg.active.provider == prov.id;
                    let stroke = if active {
                        Stroke::new(1.5, p.accent)
                    } else {
                        Stroke::new(1.0, p.border)
                    };
                    Frame::new()
                        .fill(p.tool_bg)
                        .stroke(stroke)
                        .corner_radius(CornerRadius::same(p.corner_radius))
                        .inner_margin(Margin::symmetric(16, 12))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(prov.name)
                                        .strong()
                                        .size(15.0)
                                        .color(p.text),
                                );
                                ui.label(RichText::new(prov.note).small().color(p.muted));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if active {
                                        Frame::new()
                                            .fill(p.accent_soft)
                                            .stroke(Stroke::new(1.0, p.accent))
                                            .corner_radius(CornerRadius::same(20))
                                            .inner_margin(Margin::symmetric(8, 2))
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new("● Active")
                                                        .small()
                                                        .strong()
                                                        .color(p.accent),
                                                );
                                            });
                                    }
                                });
                            });
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                if prov.needs_key {
                                    let key = cfg.keys.entry(prov.id.to_string()).or_default();
                                    dirty |= ui
                                        .add(
                                            egui::TextEdit::singleline(key)
                                                .password(true)
                                                .hint_text("API key (sk-…)")
                                                .desired_width(270.0),
                                        )
                                        .lost_focus();
                                } else {
                                    Frame::new()
                                        .fill(p.input)
                                        .stroke(Stroke::new(1.0, p.border))
                                        .corner_radius(CornerRadius::same(6))
                                        .inner_margin(Margin::symmetric(10, 5))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new("✔ No API key required")
                                                    .small()
                                                    .color(p.ok),
                                            );
                                        });
                                }
                                let model = cfg
                                    .models
                                    .entry(prov.id.to_string())
                                    .or_insert_with(|| prov.default_model.to_string());
                                dirty |= ui
                                    .add(
                                        egui::TextEdit::singleline(&mut *model)
                                            .hint_text("model id")
                                            .desired_width(240.0),
                                    )
                                    .lost_focus();

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let use_btn = Button::new(
                                        RichText::new(if active { "Selected" } else { "Use" })
                                            .strong()
                                            .color(if active {
                                                p.on_accent()
                                            } else {
                                                p.text
                                            }),
                                    )
                                    .fill(if active { p.accent } else { p.input })
                                    .stroke(Stroke::new(
                                        1.0,
                                        if active { p.accent } else { p.border },
                                    ))
                                    .min_size(vec2(78.0, 28.0));

                                    if ui.add(use_btn).clicked() {
                                        use_choice = Some((
                                            prov.id.to_string(),
                                            model.trim().to_string(),
                                        ));
                                    }
                                });
                            });
                        });
                    ui.add_space(6.0);
                }

                ui.add_space(16.0);

                // =========================================================
                // 3. CUSTOM OPENAI-COMPATIBLE ENDPOINT
                // =========================================================
                section_header(
                    ui,
                    &p,
                    "Custom OpenAI-Compatible Endpoint",
                    "Connect any server exposing POST /chat/completions (Ollama, LM Studio, vLLM, Groq, Together…).",
                );
                ui.add_space(8.0);

                let custom_active = cfg.active.provider == "custom";
                Frame::new()
                    .fill(p.tool_bg)
                    .stroke(Stroke::new(
                        if custom_active { 1.5 } else { 1.0 },
                        if custom_active { p.accent } else { p.border },
                    ))
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(16, 14))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        let c = &mut cfg.custom;
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Name").color(p.text));
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.name)
                                        .desired_width(170.0),
                                )
                                .lost_focus();
                            ui.add_space(8.0);
                            ui.label(RichText::new("Base URL").color(p.text));
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.base_url)
                                        .hint_text("http://localhost:11434/v1")
                                        .desired_width(290.0),
                                )
                                .lost_focus();
                        });
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("API key").color(p.text));
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.api_key)
                                        .password(true)
                                        .hint_text("optional")
                                        .desired_width(200.0),
                                )
                                .lost_focus();
                            ui.add_space(8.0);
                            ui.label(RichText::new("Model").color(p.text));
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.model)
                                        .hint_text("qwen2.5-coder:14b")
                                        .desired_width(200.0),
                                )
                                .lost_focus();
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let btn = Button::new(
                                    RichText::new("Use").strong().color(p.text),
                                )
                                .fill(p.input)
                                .stroke(Stroke::new(1.0, p.border))
                                .min_size(vec2(78.0, 28.0));
                                if ui.add(btn).clicked() {
                                    use_choice =
                                        Some(("custom".to_string(), c.model.trim().to_string()));
                                }
                            });
                        });
                    });

                ui.add_space(20.0);

                // =========================================================
                // 4. GENERAL & WORKSPACE
                // =========================================================
                section_header(
                    ui,
                    &p,
                    "Workspace & Permissions",
                    "Configure where new sessions run tools and whether file writes/shell commands require confirmation.",
                );
                ui.add_space(8.0);

                Frame::new()
                    .fill(p.tool_bg)
                    .stroke(Stroke::new(1.0, p.border))
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(16, 14))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Default workspace").color(p.text));
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut cfg.workspace)
                                        .desired_width(480.0),
                                )
                                .lost_focus();
                        });
                        ui.add_space(6.0);
                        dirty |= ui
                            .checkbox(
                                &mut cfg.auto_approve,
                                "Auto-approve file edits and shell commands without prompting",
                            )
                            .changed();
                    });

                ui.add_space(24.0);
            });
        });

    if let Some(preset) = preset_choice {
        app.set_theme_preset(preset);
    } else if theme_dirty {
        app.refresh_theme();
    }

    if let Some((provider, model)) = use_choice {
        if provider != "custom" {
            app.cfg.models.insert(provider.clone(), model.clone());
        }
        app.cfg.active = crate::config::Selection { provider, model };
        app.save_settings();
        app.page = Page::Chat;
    } else if dirty {
        app.save_settings();
    }
}

fn section_header(ui: &mut Ui, p: &Palette, title: &str, subtitle: &str) {
    ui.label(RichText::new(title).strong().size(17.0).color(p.text));
    ui.label(RichText::new(subtitle).small().color(p.muted));
}

fn swatch_dot(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(16.0, 12.0), egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, CornerRadius::same(3), color);
}

fn color_field(ui: &mut Ui, label: &str, rgb: &mut [u8; 3], p: &Palette) -> bool {
    let mut changed = false;
    Frame::new()
        .fill(p.input)
        .stroke(Stroke::new(1.0, p.border))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::symmetric(8, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                changed |= ui.color_edit_button_srgb(rgb).changed();
                ui.label(RichText::new(label).small().color(p.text));
            });
        });
    changed
}
