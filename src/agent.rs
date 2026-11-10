//! The agent loop: ask the model, run the tools it requests, feed results back,
//! repeat until it answers without tool calls. Runs on a worker thread and
//! reports progress through `Event`s.

use crate::llm::{self, Endpoint, Msg, ToolCall};
use crate::tools;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, channel};

const MAX_STEPS: usize = 40;

pub enum Event {
    Text(String),
    ToolStart { name: String, summary: String },
    ToolDone { name: String, output: String, is_error: bool },
    /// The worker blocks until the UI sends `true` (allow) or `false` (deny).
    Approval { description: String, reply: Sender<bool> },
    Error(String),
    /// Always sent last; carries the updated conversation back to the UI.
    Done(Vec<Msg>),
}

pub struct Job {
    pub ep: Endpoint,
    pub workspace: PathBuf,
    pub auto_approve: bool,
    pub history: Vec<Msg>,
}

pub fn run(job: Job, tx: Sender<Event>) {
    let Job {
        ep,
        workspace,
        auto_approve,
        mut history,
    } = job;
    let system = system_prompt(&workspace);
    let specs = tools::specs();

    for step in 0..MAX_STEPS {
        let reply = match llm::chat(&ep, &system, &history, &specs) {
            Ok(r) => r,
            Err(e) => {
                let _ = tx.send(Event::Error(format!("{e:#}")));
                break;
            }
        };

        if !reply.text.trim().is_empty() {
            let _ = tx.send(Event::Text(reply.text.clone()));
        }
        let calls = reply.calls;
        history.push(Msg::Assistant {
            text: reply.text,
            calls: calls.clone(),
        });
        if calls.is_empty() {
            break;
        }

        for call in &calls {
            execute(call, &workspace, auto_approve, &tx, &mut history);
        }

        if step + 1 == MAX_STEPS {
            let _ = tx.send(Event::Error(format!("Stopped after {MAX_STEPS} steps.")));
        }
    }

    let _ = tx.send(Event::Done(history));
}

fn execute(
    call: &ToolCall,
    workspace: &Path,
    auto_approve: bool,
    tx: &Sender<Event>,
    history: &mut Vec<Msg>,
) {
    let _ = tx.send(Event::ToolStart {
        name: call.name.clone(),
        summary: tools::summary(&call.name, &call.args),
    });

    let mut denied = false;
    if tools::needs_approval(&call.name) && !auto_approve {
        let (reply_tx, reply_rx) = channel();
        let _ = tx.send(Event::Approval {
            description: tools::describe(&call.name, &call.args),
            reply: reply_tx,
        });
        denied = !matches!(reply_rx.recv(), Ok(true));
    }

    let (output, is_error) = if denied {
        ("Denied by user.".to_string(), true)
    } else {
        match tools::run(workspace, &call.name, &call.args) {
            Ok(out) => (out, false),
            Err(e) => (format!("error: {e:#}"), true),
        }
    };

    let _ = tx.send(Event::ToolDone {
        name: call.name.clone(),
        output: output.clone(),
        is_error,
    });
    history.push(Msg::ToolResult {
        id: call.id.clone(),
        output,
        is_error,
    });
}

fn system_prompt(workspace: &Path) -> String {
    format!(
        "You are BeamNow, a minimal coding agent working in the user's project.\n\
         Workspace: {}\n\
         OS: {}\n\
         Read relevant files before editing, make focused changes, and verify with \
         the shell when useful. Keep replies short and plain.",
        workspace.display(),
        std::env::consts::OS
    )
}
