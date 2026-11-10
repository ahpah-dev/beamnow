//! Minimalist settings page: theme presets, collapsible custom palette, providers, and workspace.

use super::column;
use crate::app::{BeamApp, Page};
use crate::catalog;
use crate::theme::{Palette, RawPalette, ThemePreset};
use eframe::egui::{
    self, Align, Button, CornerRadius, Frame, Layout, Margin, RichText, ScrollArea, Slider,
    Stroke, Ui, vec2,
};

pub fn show(app: &mut BeamApp, ui: &mut Ui) {
    let p = app.palette;
    let mut use_choice: Option<(String, String)> = None;
    let mut preset_choice: Option<ThemePreset> = None;
    let mut theme_dirty = false;
    let mut dirty = false;

    ScrollArea::vertical()
        .id_salt("settings_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            column(ui, 660.0, |ui| {
                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    if ui.button("← Back").clicked() {
                        app.page = Page::Chat;
                    }
                    ui.heading("Settings");
                });
                ui.add_space(16.0);

                // ---------------------------------------------------------
                // Theme
                // ---------------------------------------------------------
                ui.label(RichText::new("Theme").strong().size(15.5));
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    for &preset in ThemePreset::ALL {
                        let active = app.cfg.theme.preset == preset;
                        let sample = if preset == ThemePreset::Custom {
                            app.cfg
                                .theme
                                .custom
                                .resolve(app.cfg.theme.corner_radius)
                        } else {
                            RawPalette::from_preset(preset)
                                .resolve(app.cfg.theme.corner_radius)
                        };
                        let btn = Button::new(
                            RichText::new(format!("●  {}", preset.name())).color(if active {
                                p.text
                            } else {
                                p.muted
                            }),
                        )
                        .fill(if active { p.selected } else { p.tool_bg })
                        .stroke(Stroke::new(
                            1.0,
                            if active { sample.accent } else { p.border },
                        ))
                        .corner_radius(CornerRadius::same(p.corner_radius));

                        if ui.add(btn).clicked() {
                            preset_choice = Some(preset);
                        }
                    }
                });

                ui.add_space(4.0);
                egui::CollapsingHeader::new(
                    RichText::new("Customize palette & corners")
                        .small()
                        .color(p.muted),
                )
                .id_salt("custom_palette_drawer")
                .default_open(app.cfg.theme.preset == ThemePreset::Custom)
                .show(ui, |ui| {
                    Frame::new()
                        .fill(p.tool_bg)
                        .stroke(Stroke::new(1.0, p.border))
                        .corner_radius(CornerRadius::same(p.corner_radius))
                        .inner_margin(Margin::symmetric(14, 12))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Corner radius").small().color(p.muted));
                                let mut r = app.cfg.theme.corner_radius as u32;
                                if ui.add(Slider::new(&mut r, 2..=16).suffix("px")).changed() {
                                    app.cfg.theme.corner_radius = r as u8;
                                    theme_dirty = true;
                                }
                                ui.separator();
                                if ui
                                    .checkbox(&mut app.cfg.theme.custom.dark_mode, "Dark mode")
                                    .changed()
                                {
                                    app.cfg.theme.preset = ThemePreset::Custom;
                                    theme_dirty = true;
                                }
                            });
                            ui.add_space(6.0);
                            let c = &mut app.cfg.theme.custom;
                            let mut edited = false;
                            ui.horizontal_wrapped(|ui| {
                                edited |= color_chip(ui, "Background", &mut c.bg, &p);
                                edited |= color_chip(ui, "Sidebar", &mut c.sidebar, &p);
                                edited |= color_chip(ui, "Input", &mut c.input, &p);
                                edited |= color_chip(ui, "Bubble", &mut c.user_bubble, &p);
                                edited |= color_chip(ui, "Surface", &mut c.tool_bg, &p);
                                edited |= color_chip(ui, "Accent", &mut c.accent, &p);
                                edited |= color_chip(ui, "Soft Accent", &mut c.accent_soft, &p);
                                edited |= color_chip(ui, "Border", &mut c.border, &p);
                                edited |= color_chip(ui, "Hover", &mut c.hover, &p);
                                edited |= color_chip(ui, "Selected", &mut c.selected, &p);
                                edited |= color_chip(ui, "Text", &mut c.text, &p);
                                edited |= color_chip(ui, "Muted", &mut c.muted, &p);
                                edited |= color_chip(ui, "OK", &mut c.ok, &p);
                                edited |= color_chip(ui, "Error", &mut c.err, &p);
                            });
                            if edited {
                                app.cfg.theme.preset = ThemePreset::Custom;
                                theme_dirty = true;
                            }
                        });
                });

                ui.add_space(18.0);

                // ---------------------------------------------------------
                // Providers
                // ---------------------------------------------------------
                ui.label(RichText::new("Providers").strong().size(15.5));
                ui.label(
                    RichText::new("Bring your own key. Stored locally in config.toml.")
                        .small()
                        .color(p.muted),
                );
                ui.add_space(6.0);

                let cfg = &mut app.cfg;
                for prov in catalog::POPULAR {
                    let active = cfg.active.provider == prov.id;
                    Frame::new()
                        .fill(p.tool_bg)
                        .stroke(Stroke::new(
                            1.0,
                            if active { p.accent } else { p.border },
                        ))
                        .corner_radius(CornerRadius::same(p.corner_radius))
                        .inner_margin(Margin::symmetric(14, 10))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(prov.name).strong());
                                ui.label(RichText::new(prov.note).small().color(p.muted));
                                if active {
                                    ui.label(
                                        RichText::new("● active").small().color(p.accent),
                                    );
                                }
                            });
                            ui.horizontal(|ui| {
                                if prov.needs_key {
                                    let key = cfg.keys.entry(prov.id.to_string()).or_default();
                                    dirty |= ui
                                        .add(
                                            egui::TextEdit::singleline(key)
                                                .password(true)
                                                .hint_text("API key")
                                                .desired_width(240.0),
                                        )
                                        .lost_focus();
                                } else {
                                    ui.label(
                                        RichText::new("No key needed")
                                            .small()
                                            .color(p.muted),
                                    );
                                }
                                let model = cfg
                                    .models
                                    .entry(prov.id.to_string())
                                    .or_insert_with(|| prov.default_model.to_string());
                                dirty |= ui
                                    .add(
                                        egui::TextEdit::singleline(&mut *model)
                                            .hint_text("model")
                                            .desired_width(220.0),
                                    )
                                    .lost_focus();
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let use_btn = Button::new(RichText::new("Use").small())
                                        .fill(p.input)
                                        .stroke(Stroke::new(1.0, p.border))
                                        .min_size(vec2(56.0, 24.0));
                                    if ui.add(use_btn).clicked() {
                                        use_choice = Some((
                                            prov.id.to_string(),
                                            model.trim().to_string(),
                                        ));
                                    }
                                });
                            });
                        });
                    ui.add_space(4.0);
                }

                ui.add_space(14.0);

                // ---------------------------------------------------------
                // Custom Endpoint
                // ---------------------------------------------------------
                ui.label(
                    RichText::new("Custom OpenAI-compatible")
                        .strong()
                        .size(15.5),
                );
                ui.label(
                    RichText::new("Ollama, LM Studio, vLLM, Groq, Together…")
                        .small()
                        .color(p.muted),
                );
                ui.add_space(6.0);
                Frame::new()
                    .fill(p.tool_bg)
                    .stroke(Stroke::new(1.0, p.border))
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(14, 10))
                    .show(ui, |ui| {
                        let c = &mut cfg.custom;
                        ui.horizontal(|ui| {
                            ui.label("Name");
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.name)
                                        .desired_width(140.0),
                                )
                                .lost_focus();
                            ui.label("Base URL");
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.base_url)
                                        .hint_text("https://host/v1")
                                        .desired_width(250.0),
                                )
                                .lost_focus();
                        });
                        ui.horizontal(|ui| {
                            ui.label("API key");
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.api_key)
                                        .password(true)
                                        .hint_text("optional")
                                        .desired_width(190.0),
                                )
                                .lost_focus();
                            ui.label("Model");
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut c.model)
                                        .desired_width(180.0),
                                )
                                .lost_focus();
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let use_btn = Button::new(RichText::new("Use").small())
                                    .fill(p.input)
                                    .stroke(Stroke::new(1.0, p.border))
                                    .min_size(vec2(56.0, 24.0));
                                if ui.add(use_btn).clicked() {
                                    use_choice =
                                        Some(("custom".to_string(), c.model.trim().to_string()));
                                }
                            });
                        });
                    });

                ui.add_space(14.0);

                // ---------------------------------------------------------
                // Workspace
                // ---------------------------------------------------------
                ui.label(RichText::new("Workspace").strong().size(15.5));
                ui.add_space(6.0);
                Frame::new()
                    .fill(p.tool_bg)
                    .stroke(Stroke::new(1.0, p.border))
                    .corner_radius(CornerRadius::same(p.corner_radius))
                    .inner_margin(Margin::symmetric(14, 10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Default folder");
                            dirty |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut cfg.workspace)
                                        .desired_width(440.0),
                                )
                                .lost_focus();
                        });
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

fn color_chip(ui: &mut Ui, label: &str, rgb: &mut [u8; 3], p: &Palette) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        changed |= ui.color_edit_button_srgb(rgb).changed();
        ui.label(RichText::new(label).small().color(p.muted));
    });
    changed
}
