use crate::theme::ThemeConfig;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};

/// The provider + model currently used for chat.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Selection {
    pub provider: String,
    pub model: String,
}

/// A user-defined OpenAI-compatible endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct Custom {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Config {
    pub active: Selection,
    pub auto_approve: bool,
    pub workspace: String,
    /// API keys by provider id (BYOK).
    pub keys: BTreeMap<String, String>,
    /// Model chosen per popular provider id.
    pub models: BTreeMap<String, String>,
    pub custom: Custom,
    /// Active theme preset and custom palette colors.
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // Out of the box: Kilo Code's free auto-router, no key required.
            active: Selection {
                provider: "kilo".into(),
                model: "kilo-auto/free".into(),
            },
            auto_approve: false,
            workspace: std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
            keys: BTreeMap::new(),
            models: BTreeMap::new(),
            custom: Custom {
                name: "Custom endpoint".into(),
                ..Custom::default()
            },
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("BeamNow").join("config.toml"))
    }

    /// Loads the config. A corrupt file is moved aside instead of being overwritten.
    pub fn load() -> Config {
        let Some(path) = Self::path() else {
            return Config::default();
        };
        let Ok(text) = fs::read_to_string(&path) else {
            return Config::default();
        };
        match toml::from_str::<Config>(&text) {
            Ok(cfg) => cfg,
            Err(_) => {
                let _ = fs::rename(&path, path.with_extension("toml.bak"));
                Config::default()
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path().context("no config directory available")?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(&path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}
