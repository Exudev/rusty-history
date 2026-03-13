use super::{parse_lines, HistoryParser};
use crate::command_log::CommandLog;
use anyhow::Result;
use std::path::Path;

pub struct BashParser;

impl HistoryParser for BashParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<CommandLog>> {
        parse_lines(path, true) // skip '#' comment lines
    }

    fn detect_format(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.contains("bash"))
            .unwrap_or(false)
    }
}
