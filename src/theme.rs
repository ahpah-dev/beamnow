//! Customizable theme system with built-in dark & light presets plus full user color editing.

use eframe::egui::{self, Color32, CornerRadius, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

/// Width of the centered reading column in the transcript and composer.
pub const COLUMN: f32 = 780.0;

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

    pub fn subtitle(self) -> &'static str {
        match self {
            Self::Ember => "Warm charcoal & amber",
            Self::Midnight => "Deep obsidian & electric blue",
            Self::Nord => "Arctic slate & frost cyan",
            Self::Forest => "Deep pine & emerald mint",
            Self::Rose => "Velvet plum & rose gold",
            Self::Daylight => "Crisp alabaster & cobalt",
            Self::Paper => "Warm parchment & terracotta",
            Self::Custom => "Your personalized palette",
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
            corner_radius: 10,
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
                bg: [26, 26, 30],
                sidebar: [19, 19, 22],
                input: [35, 35, 41],
                user_bubble: [44, 42, 50],
                tool_bg: [31, 31, 37],
                border: [54, 54, 64],
                hover: [45, 45, 54],
                selected: [52, 48, 58],
                text: [232, 232, 236],
                muted: [142, 142, 156],
                accent: [230, 148, 96],
                accent_soft: [58, 40, 32],
                ok: [114, 196, 132],
                err: [236, 114, 114],
            },
            ThemePreset::Midnight => Self {
                dark_mode: true,
                bg: [15, 18, 26],
                sidebar: [11, 13, 20],
                input: [22, 27, 38],
                user_bubble: [28, 36, 54],
                tool_bg: [19, 23, 33],
                border: [40, 49, 68],
                hover: [30, 38, 54],
                selected: [36, 47, 68],
                text: [228, 234, 245],
                muted: [130, 144, 168],
                accent: [96, 165, 250],
                accent_soft: [24, 40, 68],
                ok: [74, 222, 128],
                err: [248, 113, 113],
            },
            ThemePreset::Nord => Self {
                dark_mode: true,
                bg: [46, 52, 64],
                sidebar: [38, 43, 54],
                input: [59, 66, 82],
                user_bubble: [67, 76, 94],
                tool_bg: [52, 59, 73],
                border: [76, 86, 106],
                hover: [67, 76, 94],
                selected: [76, 86, 106],
                text: [236, 239, 244],
                muted: [163, 174, 194],
                accent: [136, 192, 208],
                accent_soft: [53, 72, 84],
                ok: [163, 190, 140],
                err: [191, 97, 106],
            },
            ThemePreset::Forest => Self {
                dark_mode: true,
                bg: [18, 25, 22],
                sidebar: [13, 19, 16],
                input: [25, 35, 30],
                user_bubble: [32, 46, 39],
                tool_bg: [22, 31, 27],
                border: [42, 60, 51],
                hover: [33, 47, 40],
                selected: [40, 58, 49],
                text: [226, 238, 231],
                muted: [134, 158, 144],
                accent: [86, 204, 150],
                accent_soft: [24, 52, 40],
                ok: [104, 214, 142],
                err: [238, 116, 116],
            },
            ThemePreset::Rose => Self {
                dark_mode: true,
                bg: [25, 23, 36],
                sidebar: [19, 17, 28],
                input: [33, 30, 48],
                user_bubble: [44, 39, 62],
                tool_bg: [30, 27, 43],
                border: [58, 52, 80],
                hover: [42, 38, 60],
                selected: [52, 46, 72],
                text: [234, 228, 242],
                muted: [152, 144, 174],
                accent: [235, 152, 178],
                accent_soft: [58, 36, 52],
                ok: [148, 210, 172],
                err: [235, 111, 146],
            },
            ThemePreset::Daylight => Self {
                dark_mode: false,
                bg: [248, 249, 252],
                sidebar: [238, 241, 246],
                input: [255, 255, 255],
                user_bubble: [230, 238, 252],
                tool_bg: [242, 244, 248],
                border: [214, 220, 230],
                hover: [228, 233, 242],
                selected: [218, 226, 240],
                text: [24, 28, 38],
                muted: [104, 114, 132],
                accent: [37, 99, 235],
                accent_soft: [224, 234, 254],
                ok: [22, 150, 74],
                err: [214, 48, 62],
            },
            ThemePreset::Paper => Self {
                dark_mode: false,
                bg: [247, 242, 232],
                sidebar: [237, 230, 216],
                input: [253, 250, 243],
                user_bubble: [236, 226, 208],
                tool_bg: [241, 235, 222],
                border: [214, 204, 186],
                hover: [230, 221, 204],
                selected: [222, 210, 190],
                text: [46, 38, 32],
                muted: [122, 110, 96],
                accent: [194, 92, 52],
                accent_soft: [242, 222, 210],
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
            corner_radius: corner_radius.clamp(2, 18),
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
    visuals.selection.bg_fill = p.accent.gamma_multiply(0.30);
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

    visuals.widgets.noninteractive.bg_fill = p.bg;
    visuals.widgets.noninteractive.weak_bg_fill = p.tool_bg;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.border);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.inactive.weak_bg_fill = p.input;
    visuals.widgets.inactive.bg_fill = p.input;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, p.border);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.hovered.weak_bg_fill = p.hover;
    visuals.widgets.hovered.bg_fill = p.hover;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, p.accent.gamma_multiply(0.65));
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.active.weak_bg_fill = p.selected;
    visuals.widgets.active.bg_fill = p.selected;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, p.text);

    visuals.widgets.open.weak_bg_fill = p.selected;
    visuals.widgets.open.bg_fill = p.selected;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, p.accent);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, p.text);

    let mut style = (*ctx.global_style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(11.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(12);
    style.text_styles = [
        (TextStyle::Heading, FontId::proportional(22.0)),
        (TextStyle::Body, FontId::proportional(15.0)),
        (TextStyle::Button, FontId::proportional(13.5)),
        (TextStyle::Small, FontId::proportional(12.0)),
        (TextStyle::Monospace, FontId::monospace(13.0)),
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
                corner_radius: 10,
            };
            let serialized = toml::to_string(&cfg).expect("serialize ThemeConfig");
            let restored: ThemeConfig = toml::from_str(&serialized).expect("deserialize ThemeConfig");
            assert_eq!(cfg, restored);
            let p = restored.palette();
            assert_eq!(p.corner_radius, 10);
        }
    }
}

