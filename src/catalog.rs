use serde_json::Value;
use std::time::Duration;

pub const KILO_BASE: &str = "https://api.kilo.ai/api/gateway";
pub const OPENROUTER_BASE: &str = "https://openrouter.ai/api/v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wire {
    /// `POST {base}/chat/completions`, OpenAI tool-calling format.
    OpenAi,
    /// Anthropic Messages API.
    Anthropic,
}

pub struct Provider {
    pub id: &'static str,
    pub name: &'static str,
    pub base_url: &'static str,
    pub wire: Wire,
    pub needs_key: bool,
    pub default_model: &'static str,
    pub note: &'static str,
}

/// The "Popular" providers. Custom OpenAI-compatible endpoints are handled separately.
pub const POPULAR: &[Provider] = &[
    Provider {
        id: "kilo",
        name: "Kilo Code",
        base_url: KILO_BASE,
        wire: Wire::OpenAi,
        needs_key: false,
        default_model: "kilo-auto/free",
        note: "Free models, no key needed",
    },
    Provider {
        id: "openrouter",
        name: "OpenRouter",
        base_url: OPENROUTER_BASE,
        wire: Wire::OpenAi,
        needs_key: true,
        default_model: "openrouter/free",
        note: "Free models with a free OpenRouter key",
    },
    Provider {
        id: "openai",
        name: "OpenAI",
        base_url: "https://api.openai.com/v1",
        wire: Wire::OpenAi,
        needs_key: true,
        default_model: "gpt-4.1-mini",
        note: "Bring your own key",
    },
    Provider {
        id: "nvidia",
        name: "NVIDIA NIM",
        base_url: "https://integrate.api.nvidia.com/v1",
        wire: Wire::OpenAi,
        needs_key: true,
        default_model: "meta/llama-3.3-70b-instruct",
        note: "Bring your own key",
    },
    Provider {
        id: "anthropic",
        name: "Anthropic",
        base_url: "https://api.anthropic.com/v1",
        wire: Wire::Anthropic,
        needs_key: true,
        default_model: "claude-sonnet-5-5",
        note: "Bring your own key",
    },
    Provider {
        id: "deepseek",
        name: "DeepSeek",
        base_url: "https://api.deepseek.com/v1",
        wire: Wire::OpenAi,
        needs_key: true,
        default_model: "deepseek-chat",
        note: "Bring your own key",
    },
];

pub fn find(id: &str) -> Option<&'static Provider> {
    POPULAR.iter().find(|p| p.id == id)
}

#[derive(Clone, Debug)]
pub struct FreeModel {
    pub provider: &'static str,
    pub id: String,
}

pub fn fallback_free_models() -> Vec<FreeModel> {
    vec![
        FreeModel { provider: "kilo", id: "kilo-auto/free".into() },
        FreeModel { provider: "openrouter", id: "openrouter/free".into() },
        FreeModel { provider: "openrouter", id: "google/gemma-4-31b-it:free".into() },
    ]
}

/// Fetches zero-cost, tool-capable text models from Kilo Code and OpenRouter.
/// Falls back to a built-in list when the network is unavailable.
pub fn fetch_free_models() -> Vec<FreeModel> {
    let Ok(client) = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
    else {
        return fallback_free_models();
    };

    let mut out = fetch_catalog(&client, "kilo", &format!("{KILO_BASE}/models"));
    out.extend(fetch_catalog(&client, "openrouter", &format!("{OPENROUTER_BASE}/models")));

    if !out.iter().any(|m| m.id == "kilo-auto/free") {
        out.insert(0, FreeModel { provider: "kilo", id: "kilo-auto/free".into() });
    }
    if out.iter().all(|m| m.provider != "openrouter") {
        out.extend(fallback_free_models().into_iter().filter(|m| m.provider == "openrouter"));
    }
    out
}

fn fetch_catalog(client: &reqwest::blocking::Client, provider: &'static str, url: &str) -> Vec<FreeModel> {
    let Ok(resp) = client.get(url).send() else {
        return Vec::new();
    };
    let Ok(json) = resp.json::<Value>() else {
        return Vec::new();
    };
    json["data"]
        .as_array()
        .map(|models| {
            models
                .iter()
                .filter(|m| is_free_tool_model(m))
                .filter_map(|m| m["id"].as_str())
                .map(|id| FreeModel { provider, id: id.to_string() })
                .collect()
        })
        .unwrap_or_default()
}

fn is_free_tool_model(m: &Value) -> bool {
    let is_zero = |key: &str| {
        let v = &m["pricing"][key];
        v.as_str() == Some("0") || v.as_f64() == Some(0.0)
    };
    let has_tools = m["supported_parameters"]
        .as_array()
        .is_some_and(|a| a.iter().any(|s| s.as_str() == Some("tools")));
    let text_out = m["architecture"]["output_modalities"]
        .as_array()
        .is_some_and(|a| a.iter().any(|s| s.as_str() == Some("text")));
    let stealth = m["id"].as_str().is_some_and(|id| id.starts_with("stealth/"));
    is_zero("prompt") && is_zero("completion") && has_tools && text_out && !stealth
}
