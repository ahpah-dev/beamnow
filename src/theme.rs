//! Minimalist, customizable theme system with built-in dark & light presets.

use eframe::egui::{self, Color32, CornerRadius, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

/// Width of the centered reading column in the transcript and composer.
pub const COLUMN: f32 = 720.0;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreset {
    #[default]
    Ember,
    Midnight,
    Nord,
    Forest,
    Rose,
    Daylight,
    Paper,
    Custom,
}

impl ThemePreset {
    pub const ALL: &'static [ThemePreset] = &[
        ThemePreset::Ember,
        ThemePreset::Midnight,
        ThemePreset::Nord,
        ThemePreset::Forest,
        ThemePreset::Rose,
        ThemePreset::Daylight,
        ThemePreset::Paper,
        ThemePreset::Custom,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Ember => "Ember",
            Self::Midnight => "Midnight",
            Self::Nord => "Nord",
            Self::Forest => "Forest",
            Self::Rose => "Rosé",
            Self::Daylight => "Daylight",
            Self::Paper => "Paper",
            Self::Custom => "Custom",
        }
    }
}

/// Serializable theme configuration stored in `config.toml`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct ThemeConfig {
    pub preset: ThemePreset,
    pub custom: RawPalette,
    pub corner_radius: u8,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            preset: ThemePreset::Ember,
            custom: RawPalette::from_preset(ThemePreset::Ember),
            corner_radius: 8,
        }
    }
}

impl ThemeConfig {
    pub fn palette(&self) -> Palette {
        let raw = match self.preset {
            ThemePreset::Custom => self.custom.clone(),
            preset => RawPalette::from_preset(preset),
        };
        raw.resolve(self.corner_radius)
    }
}

/// RGB-serializable palette definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct RawPalette {
    pub dark_mode: bool,
    pub bg: [u8; 3],
    pub sidebar: [u8; 3],
    pub input: [u8; 3],
    pub user_bubble: [u8; 3],
    pub tool_bg: [u8; 3],
    pub border: [u8; 3],
    pub hover: [u8; 3],
    pub selected: [u8; 3],
    pub text: [u8; 3],
    pub muted: [u8; 3],
    pub accent: [u8; 3],
    pub accent_soft: [u8; 3],
    pub ok: [u8; 3],
    pub err: [u8; 3],
}

impl Default for RawPalette {
    fn default() -> Self {
        Self::from_preset(ThemePreset::Ember)
    }
}

impl RawPalette {
    pub fn from_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::Ember | ThemePreset::Custom => Self {
                dark_mode: true,
                bg: [24, 24, 27],
                sidebar: [18, 18, 20],
                input: [32, 32, 36],
                user_bubble: [39, 39, 44],
                tool_bg: [29, 29, 33],
                border: [46, 46, 52],
                hover: [38, 38, 43],
                selected: [45, 45, 52],
                text: [228, 228, 232],
                muted: [136, 136, 148],
                accent: [224, 146, 96],
                accent_soft: [48, 36, 28],
                ok: [116, 192, 130],
                err: [232, 112, 112],
            },
            ThemePreset::Midnight => Self {
                dark_mode: true,
                bg: [14, 17, 23],
                sidebar: [10, 12, 17],
                input: [21, 25, 34],
                user_bubble: [26, 33, 46],
                tool_bg: [18, 22, 30],
                border: [34, 41, 56],
                hover: [27, 34, 47],
                selected: [33, 42, 58],
                text: [228, 233, 242],
                muted: [126, 138, 160],
                accent: [96, 165, 250],
                accent_soft: [22, 36, 60],
                ok: [82, 208, 130],
                err: [242, 112, 112],
            },
            ThemePreset::Nord => Self {
                dark_mode: true,
                bg: [46, 52, 64],
                sidebar: [39, 44, 54],
                input: [55, 62, 76],
                user_bubble: [64, 73, 90],
                tool_bg: [51, 58, 71],
                border: [69, 78, 96],
                hover: [62, 70, 86],
                selected: [70, 80, 98],
                text: [236, 239, 244],
                muted: [158, 169, 188],
                accent: [136, 192, 208],
                accent_soft: [52, 68, 79],
                ok: [163, 190, 140],
                err: [191, 97, 106],
            },
            ThemePreset::Forest => Self {
                dark_mode: true,
                bg: [17, 23, 20],
                sidebar: [13, 18, 15],
                input: [24, 32, 28],
                user_bubble: [30, 41, 35],
                tool_bg: [21, 28, 24],
                border: [37, 50, 43],
                hover: [29, 39, 34],
                selected: [35, 48, 41],
                text: [226, 236, 230],
                muted: [132, 152, 140],
                accent: [88, 198, 146],
                accent_soft: [23, 46, 36],
                ok: [104, 210, 140],
                err: [234, 114, 114],
            },
            ThemePreset::Rose => Self {
                dark_mode: true,
                bg: [25, 23, 36],
                sidebar: [19, 17, 28],
                input: [32, 29, 46],
                user_bubble: [41, 37, 58],
                tool_bg: [29, 26, 41],
                border: [48, 44, 68],
                hover: [38, 35, 54],
                selected: [46, 42, 65],
                text: [234, 228, 242],
                muted: [148, 140, 168],
                accent: [235, 152, 178],
                accent_soft: [52, 34, 48],
                ok: [148, 210, 172],
                err: [235, 111, 146],
            },
            ThemePreset::Daylight => Self {
                dark_mode: false,
                bg: [250, 250, 252],
                sidebar: [242, 243, 247],
                input: [255, 255, 255],
                user_bubble: [236, 240, 248],
                tool_bg: [244, 245, 248],
                border: [224, 227, 234],
                hover: [234, 237, 243],
                selected: [226, 231, 240],
                text: [26, 29, 36],
                muted: [112, 118, 132],
                accent: [37, 99, 235],
                accent_soft: [228, 236, 252],
                ok: [24, 148, 74],
                err: [212, 50, 62],
            },
            ThemePreset::Paper => Self {
                dark_mode: false,
                bg: [248, 244, 236],
                sidebar: [240, 234, 223],
                input: [253, 250, 244],
                user_bubble: [238, 230, 216],
                tool_bg: [243, 237, 227],
                border: [222, 213, 198],
                hover: [234, 226, 212],
                selected: [226, 216, 200],
                text: [44, 38, 32],
                muted: [124, 113, 100],
                accent: [192, 92, 54],
                accent_soft: [242, 225, 214],
                ok: [46, 138, 78],
                err: [196, 58, 58],
            },
        }
    }

    pub fn resolve(&self, corner_radius: u8) -> Palette {
        let c = |rgb: [u8; 3]| Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
        Palette {
            dark_mode: self.dark_mode,
            bg: c(self.bg),
            sidebar: c(self.sidebar),
            input: c(self.input),
            user_bubble: c(self.user_bubble),
            tool_bg: c(self.tool_bg),
            border: c(self.border),
            hover: c(self.hover),
            selected: c(self.selected),
            text: c(self.text),
            muted: c(self.muted),
            accent: c(self.accent),
            accent_soft: c(self.accent_soft),
            ok: c(self.ok),
            err: c(self.err),
            corner_radius: corner_radius.clamp(2, 16),
        }
    }
}

/// Resolved runtime palette used across all views.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Palette {
    pub dark_mode: bool,
    pub bg: Color32,
    pub sidebar: Color32,
    pub input: Color32,
    pub user_bubble: Color32,
    pub tool_bg: Color32,
    pub border: Color32,
    pub hover: Color32,
    pub selected: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub ok: Color32,
    pub err: Color32,
    pub corner_radius: u8,
}

impl Palette {
    /// Text color that contrasts well when drawn on top of `accent`.
    pub fn on_accent(&self) -> Color32 {
        let [r, g, b, _] = self.accent.to_array();
        let luma = 0.299 * (r as f32) + 0.587 * (g as f32) + 0.114 * (b as f32);
        if luma > 150.0 {
            Color32::from_rgb(20, 20, 24)
        } else {
            Color32::from_rgb(250, 250, 252)
        }
    }
}

pub fn apply(ctx: &egui::Context, p: &Palette) {
    ctx.set_theme(if p.dark_mode {
        egui::ThemePreference::Dark
    } else {
        egui::ThemePreference::Light
    });

    let mut visuals = if p.dark_mode {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.panel_fill = p.bg;
    visuals.window_fill = p.bg;
    visuals.window_stroke = Stroke::new(1.0, p.border);
    visuals.faint_bg_color = p.tool_bg;
    visuals.extreme_bg_color = p.input;
    visuals.code_bg_color = p.tool_bg;
    visuals.override_text_color = Some(p.text);
    visuals.hyperlink_color = p.accent;
    visuals.selection.bg_fill = p.accent.gamma_multiply(0.28);
    visuals.selection.stroke = Stroke::new(1.0, p.accent);

    let radius = CornerRadius::same(p.corner_radius);
    for w in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        w.corner_radius = radius;
    }

    // Minimalist low-chrome widgets: borderless at rest, soft surface fills on interaction.
    visuals.widgets.noninteractive.bg_fill = p.bg;
    visuals.widgets.noninteractive.weak_bg_fill = p.tool_bg;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.hovered.weak_bg_fill = p.hover;
    visuals.widgets.hovered.bg_fill = p.hover;
    visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.active.weak_bg_fill = p.selected;
    visuals.widgets.active.bg_fill = p.selected;
    visuals.widgets.active.bg_stroke = Stroke::NONE;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.open.weak_bg_fill = p.selected;
    visuals.widgets.open.bg_fill = p.selected;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, p.border);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, p.text);

    let mut style = (*ctx.global_style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.spacing.window_margin = egui::Margin::same(10);
    style.text_styles = [
        (TextStyle::Heading, FontId::proportional(20.0)),
        (TextStyle::Body, FontId::proportional(14.5)),
        (TextStyle::Button, FontId::proportional(13.5)),
        (TextStyle::Small, FontId::proportional(12.0)),
        (TextStyle::Monospace, FontId::monospace(12.5)),
    ]
    .into();
    ctx.set_global_style(style);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_resolve_and_roundtrip_toml() {
        for &preset in ThemePreset::ALL {
            let cfg = ThemeConfig {
                preset,
                custom: RawPalette::from_preset(preset),
                corner_radius: 8,
            };
            let serialized = toml::to_string(&cfg).expect("serialize ThemeConfig");
            let restored: ThemeConfig =
                toml::from_str(&serialized).expect("deserialize ThemeConfig");
            assert_eq!(cfg, restored);
            let p = restored.palette();
            assert_eq!(p.corner_radius, 8);
        }
    }
}
