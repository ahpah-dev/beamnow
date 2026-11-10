//! Provider-neutral chat messages plus the two wire formats BeamNow speaks:
//! OpenAI-style `/chat/completions` (OpenRouter, OpenAI, NVIDIA NIM, DeepSeek,
//! Kilo Code, custom endpoints) and Anthropic's Messages API.

use crate::catalog::Wire;
use crate::tools::clip;
use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::time::Duration;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub args: Value,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Msg {
    User(String),
    Assistant { text: String, calls: Vec<ToolCall> },
    ToolResult { id: String, output: String, is_error: bool },
}

pub struct ToolSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub schema: Value,
}

#[derive(Default)]
pub struct Reply {
    pub text: String,
    pub calls: Vec<ToolCall>,
}

#[derive(Clone, Debug)]
pub struct Endpoint {
    pub wire: Wire,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

pub fn chat(ep: &Endpoint, system: &str, msgs: &[Msg], tools: &[ToolSpec]) -> Result<Reply> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(300))
        .build()?;
    match ep.wire {
        Wire::OpenAi => openai_chat(&client, ep, system, msgs, tools),
        Wire::Anthropic => anthropic_chat(&client, ep, system, msgs, tools),
    }
}

fn post(
    client: &reqwest::blocking::Client,
    url: &str,
    headers: Vec<(&str, String)>,
    body: &Value,
) -> Result<Value> {
    let mut req = client.post(url).json(body);
    for (k, v) in headers {
        req = req.header(k, v);
    }
    let resp = req
        .send()
        .with_context(|| format!("request to {url} failed"))?;
    let status = resp.status();
    let text = resp.text()?;
    if !status.is_success() {
        bail!("HTTP {}: {}", status.as_u16(), clip(&text, 800));
    }
    serde_json::from_str(&text).with_context(|| format!("invalid JSON from {url}"))
}

fn openai_chat(
    client: &reqwest::blocking::Client,
    ep: &Endpoint,
    system: &str,
    msgs: &[Msg],
    tools: &[ToolSpec],
) -> Result<Reply> {
    let mut messages = vec![json!({"role": "system", "content": system})];
    for m in msgs {
        match m {
            Msg::User(t) => messages.push(json!({"role": "user", "content": t})),
            Msg::Assistant { text, calls } => {
                let content = if text.is_empty() { Value::Null } else { json!(text) };
                let mut o = json!({"role": "assistant", "content": content});
                if !calls.is_empty() {
                    o["tool_calls"] = Value::Array(
                        calls
                            .iter()
                            .map(|c| {
                                json!({
                                    "id": c.id,
                                    "type": "function",
                                    "function": {"name": c.name, "arguments": c.args.to_string()}
                                })
                            })
                            .collect(),
                    );
                }
                messages.push(o);
            }
            Msg::ToolResult { id, output, .. } => {
                messages.push(json!({"role": "tool", "tool_call_id": id, "content": output}))
            }
        }
    }

    let mut body = json!({"model": ep.model, "messages": messages});
    if !tools.is_empty() {
        body["tools"] = Value::Array(
            tools
                .iter()
                .map(|t| {
                    json!({"type": "function", "function": {
                        "name": t.name, "description": t.description, "parameters": t.schema
                    }})
                })
                .collect(),
        );
    }

    let mut headers = Vec::new();
    if !ep.api_key.is_empty() {
        headers.push(("Authorization", format!("Bearer {}", ep.api_key)));
    }
    let url = format!("{}/chat/completions", ep.base_url.trim_end_matches('/'));
    let v = post(client, &url, headers, &body)?;

    let msg = &v["choices"][0]["message"];
    let text = msg["content"].as_str().unwrap_or("").to_string();
    let calls: Vec<ToolCall> = msg["tool_calls"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|c| ToolCall {
                    id: c["id"].as_str().unwrap_or("call").to_string(),
                    name: c["function"]["name"].as_str().unwrap_or("").to_string(),
                    args: match &c["function"]["arguments"] {
                        Value::String(s) => serde_json::from_str(s).unwrap_or(Value::Null),
                        other => other.clone(),
                    },
                })
                .collect()
        })
        .unwrap_or_default();

    if text.trim().is_empty() && calls.is_empty() {
        bail!("empty response from model: {}", clip(&v.to_string(), 400));
    }
    Ok(Reply { text, calls })
}

fn anthropic_chat(
    client: &reqwest::blocking::Client,
    ep: &Endpoint,
    system: &str,
    msgs: &[Msg],
    tools: &[ToolSpec],
) -> Result<Reply> {
    let mut messages: Vec<Value> = Vec::new();
    for m in msgs {
        match m {
            Msg::User(t) => {
                messages.push(json!({"role": "user", "content": [{"type": "text", "text": t}]}))
            }
            Msg::Assistant { text, calls } => {
                let mut blocks = Vec::new();
                if !text.is_empty() {
                    blocks.push(json!({"type": "text", "text": text}));
                }
                for c in calls {
                    let input = if c.args.is_object() { c.args.clone() } else { json!({}) };
                    blocks.push(json!({"type": "tool_use", "id": c.id, "name": c.name, "input": input}));
                }
                if !blocks.is_empty() {
                    messages.push(json!({"role": "assistant", "content": blocks}));
                }
            }
            Msg::ToolResult { id, output, is_error } => {
                let block = json!({
                    "type": "tool_result",
                    "tool_use_id": id,
                    "content": output,
                    "is_error": is_error
                });
                // Consecutive tool results share one user turn, as the API expects.
                let joinable = messages.last().is_some_and(|last| {
                    last["role"] == "user"
                        && last["content"][0]["type"] == "tool_result"
                });
                if joinable {
                    if let Some(arr) = messages.last_mut().and_then(|l| l["content"].as_array_mut()) {
                        arr.push(block);
                    }
                } else {
                    messages.push(json!({"role": "user", "content": [block]}));
                }
            }
        }
    }

    let tool_defs: Vec<Value> = tools
        .iter()
        .map(|t| json!({"name": t.name, "description": t.description, "input_schema": t.schema}))
        .collect();
    let body = json!({
        "model": ep.model,
        "max_tokens": 8192,
        "system": system,
        "messages": messages,
        "tools": tool_defs,
    });
    let headers = vec![
        ("x-api-key", ep.api_key.clone()),
        ("anthropic-version", "2023-06-01".to_string()),
    ];
    let url = format!("{}/messages", ep.base_url.trim_end_matches('/'));
    let v = post(client, &url, headers, &body)?;

    let mut text = String::new();
    let mut calls = Vec::new();
    for b in v["content"].as_array().cloned().unwrap_or_default() {
        match b["type"].as_str() {
            Some("text") => text.push_str(b["text"].as_str().unwrap_or("")),
            Some("tool_use") => calls.push(ToolCall {
                id: b["id"].as_str().unwrap_or("call").to_string(),
                name: b["name"].as_str().unwrap_or("").to_string(),
                args: b["input"].clone(),
            }),
            _ => {}
        }
    }
    if text.trim().is_empty() && calls.is_empty() {
        bail!("empty response from model: {}", clip(&v.to_string(), 400));
    }
    Ok(Reply { text, calls })
}

#[cfg(test)]
mod live_tests {
    use super::*;

    /// Live check against Kilo Code's keyless free gateway: `cargo test live_kilo -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn live_kilo_tool_round_trip() {
        let ep = Endpoint {
            wire: Wire::OpenAi,
            base_url: crate::catalog::KILO_BASE.into(),
            api_key: String::new(),
            model: "kilo-auto/free".into(),
        };
        let tools = crate::tools::specs();
        let mut msgs = vec![Msg::User("Use the list_dir tool on the path '.' and then say done.".into())];
        let first = chat(&ep, "You are a test.", &msgs, &tools).expect("first call");
        println!("first text={:?} calls={:?}", first.text, first.calls.iter().map(|c| &c.name).collect::<Vec<_>>());
        if first.calls.is_empty() {
            return;
        }
        msgs.push(Msg::Assistant { text: first.text.clone(), calls: first.calls.clone() });
        for c in &first.calls {
            msgs.push(Msg::ToolResult { id: c.id.clone(), output: "Cargo.toml\nsrc/".into(), is_error: false });
        }
        let second = chat(&ep, "You are a test.", &msgs, &tools).expect("second call");
        println!("second text={:?}", second.text);
    }
}
