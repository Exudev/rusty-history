pub mod bash;
pub mod zsh;

use anyhow::Result;
use std::path::Path;

use crate::commandLog::CommandLog;

/// Trait for parsing different shell history formats
pub trait HistoryParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<CommandLog>>;
    fn detect_format(&self, path: &Path) -> bool;
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

