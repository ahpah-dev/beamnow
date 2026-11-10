//! Tools the agent can call. Mutating tools (write, edit, shell) need approval
//! unless auto-approve is on.

use crate::llm::ToolSpec;
use anyhow::{Context, Result, anyhow, bail};
use regex::Regex;
use serde_json::{Value, json};
use std::fmt::Write as _;
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

const MAX_READ: usize = 200_000;
const MAX_OUT: usize = 30_000;
const MAX_HITS: usize = 300;
const MAX_FILES: usize = 20_000;

pub fn specs() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "read_file",
            description: "Read a text file. Returns numbered lines.",
            schema: json!({"type": "object", "properties": {
                "path": {"type": "string", "description": "Path, relative to the workspace or absolute"}
            }, "required": ["path"]}),
        },
        ToolSpec {
            name: "write_file",
            description: "Create or overwrite a file with the given content.",
            schema: json!({"type": "object", "properties": {
                "path": {"type": "string"},
                "content": {"type": "string"}
            }, "required": ["path", "content"]}),
        },
        ToolSpec {
            name: "edit_file",
            description: "Replace an exact string in a file. old_string must occur exactly once unless replace_all is true.",
            schema: json!({"type": "object", "properties": {
                "path": {"type": "string"},
                "old_string": {"type": "string"},
                "new_string": {"type": "string"},
                "replace_all": {"type": "boolean"}
            }, "required": ["path", "old_string", "new_string"]}),
        },
        ToolSpec {
            name: "list_dir",
            description: "List the entries of a directory. Directories end with /.",
            schema: json!({"type": "object", "properties": {
                "path": {"type": "string"}
            }}),
        },
        ToolSpec {
            name: "grep",
            description: "Search files for a regular expression. Returns path:line: text matches.",
            schema: json!({"type": "object", "properties": {
                "pattern": {"type": "string"},
                "path": {"type": "string"}
            }, "required": ["pattern"]}),
        },
        ToolSpec {
            name: "run_shell",
            description: "Run a shell command in the workspace. Returns the exit code and output.",
            schema: json!({"type": "object", "properties": {
                "command": {"type": "string"},
                "timeout_secs": {"type": "integer", "description": "1-600, default 120"}
            }, "required": ["command"]}),
        },
    ]
}

pub fn needs_approval(name: &str) -> bool {
    matches!(name, "write_file" | "edit_file" | "run_shell")
}

/// Short one-line description shown in the transcript.
pub fn summary(name: &str, args: &Value) -> String {
    let s = |k: &str| args[k].as_str().unwrap_or("").to_string();
    match name {
        "read_file" | "write_file" | "edit_file" | "list_dir" => s("path"),
        "grep" => s("pattern"),
        "run_shell" => s("command"),
        _ => args.to_string(),
    }
}

/// Longer description shown in the approval prompt.
pub fn describe(name: &str, args: &Value) -> String {
    let s = |k: &str| args[k].as_str().unwrap_or("").to_string();
    match name {
        "write_file" => format!("Write {} ({} bytes)", s("path"), s("content").len()),
        "edit_file" => format!(
            "Edit {}\n- {}\n+ {}",
            s("path"),
            clip(&s("old_string"), 300),
            clip(&s("new_string"), 300)
        ),
        "run_shell" => format!("Run:\n{}", s("command")),
        _ => summary(name, args),
    }
}

pub fn run(ws: &Path, name: &str, args: &Value) -> Result<String> {
    match name {
        "read_file" => read_file(ws, args),
        "write_file" => write_file(ws, args),
        "edit_file" => edit_file(ws, args),
        "list_dir" => list_dir(ws, args),
        "grep" => grep(ws, args),
        "run_shell" => run_shell(ws, args),
        other => bail!("unknown tool: {other}"),
    }
}

fn arg<'a>(args: &'a Value, key: &str) -> Result<&'a str> {
    args[key]
        .as_str()
        .ok_or_else(|| anyhow!("missing string argument '{key}'"))
}

fn resolve(ws: &Path, p: &str) -> PathBuf {
    let pb = PathBuf::from(p);
    if pb.is_absolute() { pb } else { ws.join(pb) }
}

fn read_file(ws: &Path, args: &Value) -> Result<String> {
    let path = resolve(ws, arg(args, "path")?);
    let text = fs::read_to_string(&path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    let mut out = String::new();
    for (i, line) in text.lines().enumerate() {
        let _ = writeln!(out, "{:>5}| {}", i + 1, line);
        if out.len() > MAX_READ {
            out.push_str("… (truncated)\n");
            break;
        }
    }
    Ok(out)
}

fn write_file(ws: &Path, args: &Value) -> Result<String> {
    let path = resolve(ws, arg(args, "path")?);
    let content = arg(args, "content")?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&path, content).with_context(|| format!("cannot write {}", path.display()))?;
    Ok(format!("Wrote {} bytes to {}", content.len(), path.display()))
}

fn edit_file(ws: &Path, args: &Value) -> Result<String> {
    let path = resolve(ws, arg(args, "path")?);
    let old = arg(args, "old_string")?;
    let new = arg(args, "new_string")?;
    let replace_all = args["replace_all"].as_bool().unwrap_or(false);
    if old.is_empty() {
        bail!("old_string must not be empty (use write_file to create a file)");
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    let count = text.matches(old).count();
    if count == 0 {
        bail!("old_string not found in {}", path.display());
    }
    if count > 1 && !replace_all {
        bail!(
            "old_string occurs {count} times in {}; add surrounding context or set replace_all",
            path.display()
        );
    }
    let updated = if replace_all {
        text.replace(old, new)
    } else {
        text.replacen(old, new, 1)
    };
    fs::write(&path, updated)?;
    let n = if replace_all { count } else { 1 };
    Ok(format!("Edited {} ({n} replacement{})", path.display(), if n == 1 { "" } else { "s" }))
}

fn list_dir(ws: &Path, args: &Value) -> Result<String> {
    let path = resolve(ws, args["path"].as_str().unwrap_or("."));
    let mut entries: Vec<(bool, String)> = fs::read_dir(&path)
        .with_context(|| format!("cannot list {}", path.display()))?
        .filter_map(|e| e.ok())
        .map(|e| {
            let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
            (is_dir, e.file_name().to_string_lossy().into_owned())
        })
        .collect();
    entries.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    if entries.is_empty() {
        return Ok("(empty)\n".into());
    }
    let mut out = String::new();
    for (is_dir, name) in entries.iter().take(500) {
        out.push_str(name);
        if *is_dir {
            out.push('/');
        }
        out.push('\n');
    }
    if entries.len() > 500 {
        let _ = writeln!(out, "… {} more", entries.len() - 500);
    }
    Ok(out)
}

fn grep(ws: &Path, args: &Value) -> Result<String> {
    let re = Regex::new(arg(args, "pattern")?).context("invalid regex")?;
    let root = resolve(ws, args["path"].as_str().unwrap_or("."));
    let mut out = String::new();
    let mut hits = 0;
    let mut files = 0;
    grep_dir(&root, &re, 0, &mut out, &mut hits, &mut files);
    if out.is_empty() {
        out.push_str("No matches.\n");
    }
    if hits >= MAX_HITS {
        let _ = writeln!(out, "… stopped at {MAX_HITS} matches");
    }
    Ok(out)
}

fn grep_dir(
    dir: &Path,
    re: &Regex,
    depth: usize,
    out: &mut String,
    hits: &mut usize,
    files: &mut usize,
) {
    if depth > 16 || *hits >= MAX_HITS || *files >= MAX_FILES {
        return;
    }
    let Ok(read) = fs::read_dir(dir) else { return };
    for entry in read.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let Ok(kind) = entry.file_type() else { continue };
        if kind.is_dir() {
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            grep_dir(&path, re, depth + 1, out, hits, files);
        } else if kind.is_file() {
            let Ok(meta) = entry.metadata() else { continue };
            if meta.len() > 1_000_000 {
                continue;
            }
            // Non-UTF-8 (binary) files fail here and are skipped.
            let Ok(text) = fs::read_to_string(&path) else { continue };
            *files += 1;
            for (i, line) in text.lines().enumerate() {
                if re.is_match(line) {
                    *hits += 1;
                    let _ = writeln!(out, "{}:{}: {}", path.display(), i + 1, clip(line.trim(), 200));
                    if *hits >= MAX_HITS {
                        return;
                    }
                }
            }
        }
    }
}

fn run_shell(ws: &Path, args: &Value) -> Result<String> {
    let cmd = arg(args, "command")?;
    let secs = args["timeout_secs"].as_u64().unwrap_or(120).clamp(1, 600);
    let timeout = Duration::from_secs(secs);

    let mut command = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.arg("/C");
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c");
        c
    };
    command
        .arg(cmd)
        .current_dir(ws)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().context("failed to start shell")?;

    let mut so = child.stdout.take().context("no stdout handle")?;
    let mut se = child.stderr.take().context("no stderr handle")?;
    let out_t = thread::spawn(move || {
        let mut b = Vec::new();
        let _ = so.read_to_end(&mut b);
        b
    });
    let err_t = thread::spawn(move || {
        let mut b = Vec::new();
        let _ = se.read_to_end(&mut b);
        b
    });

    let start = Instant::now();
    let status = loop {
        if let Some(st) = child.try_wait()? {
            break st;
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            // Reader threads are detached: grandchildren may still hold the pipes.
            bail!("command timed out after {secs}s");
        }
        thread::sleep(Duration::from_millis(50));
    };

    let stdout = out_t.join().unwrap_or_default();
    let stderr = err_t.join().unwrap_or_default();
    let code = status
        .code()
        .map(|c| c.to_string())
        .unwrap_or_else(|| "signal".into());

    let mut out = format!("exit code: {code}\n");
    let so = String::from_utf8_lossy(&stdout);
    let se = String::from_utf8_lossy(&stderr);
    if !so.trim().is_empty() {
        out.push_str("--- stdout ---\n");
        out.push_str(&clip(&so, MAX_OUT));
        out.push('\n');
    }
    if !se.trim().is_empty() {
        out.push_str("--- stderr ---\n");
        out.push_str(&clip(&se, MAX_OUT));
        out.push('\n');
    }
    Ok(out)
}

/// Truncates to `max` characters, marking the cut.
pub fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let head: String = s.chars().take(max).collect();
        format!("{head}\n… (truncated)")
    }
}
