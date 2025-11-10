use super::HistoryParser;
use crate::commandLog::CommandLog;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ZshParser;

impl HistoryParser for ZshParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<CommandLog>> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open history file: {}", path.display()))?;
        let reader = BufReader::new(file);
        let mut commands = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();
        let cwd = home.to_string_lossy().to_string();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            
            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Zsh extended history format: : timestamp:duration:command
            if trimmed.starts_with(':') {
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
                // Simple format without timestamp
                commands.push(CommandLog::new(
                    Utc::now(),
                    trimmed.to_string(),
                    cwd.clone(),
                ));
            }
        }

        Ok(commands)
    }

    fn detect_format(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.contains("zsh"))
            .unwrap_or(false)
    }
}

