//! Chat sessions: what the transcript shows, what the model sees, and how they
//! are stored on disk (one JSON file per session).

use crate::llm::Msg;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub const DEFAULT_TITLE: &str = "New chat";

/// One row of the transcript.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Entry {
    User(String),
    Assistant(String),
    Tool {
        name: String,
        summary: String,
        /// `None` while the tool is still running; `(output, is_error)` after.
        output: Option<(String, bool)>,
    },
    Notice(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub workspace: String,
    /// Unix seconds of the last change; used for sorting.
    pub updated: u64,
    pub entries: Vec<Entry>,
    /// Conversation sent to the model.
    pub history: Vec<Msg>,
}

impl Session {
    pub fn new(workspace: String) -> Self {
        let now = unix_millis();
        Self {
            id: now.to_string(),
            title: DEFAULT_TITLE.into(),
            workspace,
            updated: now / 1000,
            entries: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn touch(&mut self) {
        self.updated = unix_millis() / 1000;
    }

    /// Names an untitled session after its first message.
    pub fn name_from(&mut self, text: &str) {
        if self.title == DEFAULT_TITLE {
            let first = text.lines().next().unwrap_or("").trim();
            let title: String = first.chars().take(48).collect();
            if !title.is_empty() {
                self.title = title;
            }
        }
    }
}

fn unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn dir() -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join("BeamNow").join("sessions"))
}

pub fn save(session: &Session) -> Result<()> {
    let dir = dir().context("no data directory available")?;
    fs::create_dir_all(&dir)?;
    let text = serde_json::to_string(session)?;
    fs::write(dir.join(format!("{}.json", session.id)), text)?;
    Ok(())
}

/// Loads every saved session, newest first. Unreadable files are skipped.
pub fn load_all() -> Vec<Session> {
    let Some(dir) = dir() else {
        return Vec::new();
    };
    let mut sessions: Vec<Session> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .filter_map(|e| fs::read_to_string(e.path()).ok())
        .filter_map(|text| serde_json::from_str(&text).ok())
        .collect();
    sessions.sort_by(|a: &Session, b: &Session| b.updated.cmp(&a.updated));
    sessions
}

pub fn delete(id: &str) {
    if let Some(dir) = dir() {
        let _ = fs::remove_file(dir.join(format!("{id}.json")));
    }
}
