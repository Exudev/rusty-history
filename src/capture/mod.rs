pub mod bash;
pub mod zsh;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::command_log::CommandLog;

/// Trait for parsing different shell history formats
pub trait HistoryParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<CommandLog>>;
    fn detect_format(&self, path: &Path) -> bool;
}

/// Shared line-by-line parser for the extended history format used by both
/// Bash and Zsh (`: timestamp:elapsed;command`).
/// Pass `skip_comments = true` for Bash, which uses `#` comment lines.
pub fn parse_lines(path: &Path, skip_comments: bool) -> Result<Vec<CommandLog>> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open history file: {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut commands = Vec::new();
    let home = dirs::home_dir().unwrap_or_default();
    let cwd = home.to_string_lossy().to_string();

    // Fallback timestamp for lines without an explicit timestamp.
    // The file's mtime is a real historical date, unlike Utc::now() which
    // would stamp every sync run with today's date.
    let fallback_ts: DateTime<Utc> = std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .map(DateTime::from)
        .unwrap_or_else(Utc::now);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || (skip_comments && trimmed.starts_with('#')) {
            continue;
        }

        // Extended history format: ": timestamp:elapsed;command"
        if trimmed.starts_with(':') {
            // Split into at most 3 parts on ':' → ["", " timestamp", "elapsed;command"]
            let parts: Vec<&str> = trimmed.splitn(3, ':').collect();
            if parts.len() == 3 {
                if let Ok(timestamp) = parts[1].trim().parse::<i64>() {
                    let command = parts[2].splitn(2, ';').nth(1).unwrap_or("").trim();
                    if !command.is_empty() {
                        let dt = DateTime::from_timestamp(timestamp, 0)
                            .unwrap_or_else(Utc::now);
                        commands.push(CommandLog::new(dt, command.to_string(), cwd.clone()));
                    }
                }
            }
        } else {
            // Simple format — no timestamp in file, use mtime as best-effort fallback
            commands.push(CommandLog::new(fallback_ts, trimmed.to_string(), cwd.clone()));
        }
    }

    Ok(commands)
}

/// Auto-detect and parse shell history files
pub fn parse_history_file(path: &Path) -> Result<Vec<CommandLog>> {
    let bash_parser = bash::BashParser;
    let zsh_parser = zsh::ZshParser;

    if bash_parser.detect_format(path) {
        bash_parser.parse_file(path)
    } else if zsh_parser.detect_format(path) {
        zsh_parser.parse_file(path)
    } else {
        // Default to bash format
        bash_parser.parse_file(path)
    }
}

/// Find common history file locations
pub fn find_history_files() -> Vec<std::path::PathBuf> {
    let home = dirs::home_dir().unwrap_or_default();
    let mut files = Vec::new();

    // Common history file locations
    let candidates = vec![
        home.join(".bash_history"),
        home.join(".zsh_history"),
        home.join(".history"),
        home.join(".local/share/fish/fish_history"),
    ];

    for path in candidates {
        if path.exists() {
            files.push(path);
        }
    }

    files
}

/// Detect the user's default shell
pub fn detect_shell() -> Option<String> {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| {
            std::path::Path::new(&s)
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
        })
        .or_else(|| {
            // Fallback: check common shell executables
            let shells = vec!["bash", "zsh", "fish", "sh"];
            for shell in shells {
                if which::which(shell).is_ok() {
                    return Some(shell.to_string());
                }
            }
            None
        })
}

