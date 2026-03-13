use crate::command_log::CommandLog;

/// Represents the frequency of a command
#[derive(Debug, Clone)]
pub struct CommandFrequency {
    pub command: String,
    pub count: u64,
    pub percentage: f64,
}

impl CommandFrequency {
    pub fn new(command: String, count: u64, total: usize) -> Self {
        Self {
            command,
            count,
            percentage: if total > 0 {
                (count as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
}

/// Calculate frequency for base commands (first word)
pub fn analyze_base_commands(commands: &[CommandLog]) -> Vec<CommandFrequency> {
    use std::collections::HashMap;

    let mut counts: HashMap<String, u64> = HashMap::new();

    for cmd in commands {
        let base = cmd.base_command();
        *counts.entry(base).or_insert(0) += 1;
    }

    let mut frequencies: Vec<CommandFrequency> = counts
        .into_iter()
        .map(|(command, count)| {
            CommandFrequency::new(command, count, commands.len())
        })
        .collect();

    frequencies.sort_by(|a, b| b.count.cmp(&a.count));
    frequencies
}

