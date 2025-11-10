use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Represents a single command execution with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLog {
    pub timestamp: DateTime<Utc>,
    pub command_text: String,
    pub working_directory: String,
    pub exit_code: Option<i32>,
    pub session_id: Option<String>,
}

impl CommandLog {
    pub fn new(
        timestamp: DateTime<Utc>,
        command_text: String,
        working_directory: String,
    ) -> Self {
        Self {
            timestamp,
            command_text,
            working_directory,
            exit_code: None,
            session_id: None,
        }
    }

    /// Extract the base command (first word) from the command text
    pub fn base_command(&self) -> String {
        self.command_text
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    }

    /// Check if command is likely an alias or function
    pub fn is_alias(&self) -> bool {
        // Simple heuristic: if it doesn't contain a path separator, might be alias
        !self.command_text.contains('/') && !self.command_text.starts_with('.')
    }
}
