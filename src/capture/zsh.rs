use super::{parse_lines, HistoryParser};
use crate::command_log::CommandLog;
use anyhow::Result;
use std::path::Path;

pub struct ZshParser;

impl HistoryParser for ZshParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<CommandLog>> {
        parse_lines(path, false) // zsh history has no comment lines to skip
    }

    fn detect_format(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.contains("zsh"))
            .unwrap_or(false)
    }
}
