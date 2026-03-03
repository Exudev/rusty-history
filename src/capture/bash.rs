use super::HistoryParser;
use crate::commandLog::CommandLog;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct BashParser;

impl HistoryParser for BashParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<CommandLog>> {
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
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Bash history format: just the command, no timestamp by default
            // Extended history format: : timestamp:duration:command
            if trimmed.starts_with(':') {
                // Extended format
                let parts: Vec<&str> = trimmed.splitn(4, ':').collect();
                if parts.len() >= 4 {
                    if let Ok(timestamp) = parts[1].parse::<i64>() {
                        let command = parts[3];
                        let dt = DateTime::from_timestamp(timestamp, 0)
                            .unwrap_or_else(Utc::now);
                        commands.push(CommandLog::new(
                            dt,
                            command.to_string(),
                            cwd.clone(),
                        ));
                    }
                }
            } else {
                // Simple format — no timestamp in file, use mtime as best-effort fallback
                commands.push(CommandLog::new(
                    fallback_ts,
                    trimmed.to_string(),
                    cwd.clone(),
                ));
            }
        }

        Ok(commands)
    }

    fn detect_format(&self, path: &Path) -> bool {
        // Check if file exists and has .bash_history in name or starts with bash format
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.contains("bash"))
            .unwrap_or(false)
    }
}

