pub mod frequency;
pub mod patterns;

use crate::command_log::CommandLog;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub use frequency::CommandFrequency;
pub use patterns::TimePatterns;

/// Analyze command frequency
pub fn analyze_frequency(commands: &[CommandLog]) -> Vec<CommandFrequency> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let mut base_counts: HashMap<String, u64> = HashMap::new();

    for cmd in commands {
        *counts.entry(cmd.command_text.clone()).or_insert(0) += 1;
        *base_counts.entry(cmd.base_command()).or_insert(0) += 1;
    }

    let mut frequencies: Vec<CommandFrequency> = counts
        .into_iter()
        .map(|(command, count)| CommandFrequency {
            command,
            count,
            percentage: (count as f64 / commands.len() as f64) * 100.0,
        })
        .collect();

    frequencies.sort_by(|a, b| b.count.cmp(&a.count));
    frequencies
}

/// Get top N commands
pub fn top_commands(commands: &[CommandLog], n: usize) -> Vec<CommandFrequency> {
    let mut frequencies = analyze_frequency(commands);
    frequencies.truncate(n);
    frequencies
}

/// Analyze time-based patterns
pub fn analyze_time_patterns(commands: &[CommandLog]) -> TimePatterns {
    patterns::analyze_time_patterns(commands)
}

/// Generate insights for "Wrapped" style report
pub struct Insights {
    pub total_commands: usize,
    pub unique_commands: usize,
    pub most_used_command: String,
    pub most_used_count: u64,
    pub busiest_hour: u8,
    pub busiest_day: String,
    pub total_days: i64,
    pub first_command_date: Option<DateTime<Utc>>,
    pub last_command_date: Option<DateTime<Utc>>,
}

pub fn generate_insights(commands: &[CommandLog]) -> Insights {
    if commands.is_empty() {
        return Insights {
            total_commands: 0,
            unique_commands: 0,
            most_used_command: String::new(),
            most_used_count: 0,
            busiest_hour: 0,
            busiest_day: String::new(),
            total_days: 0,
            first_command_date: None,
            last_command_date: None,
        };
    }

    let frequencies = analyze_frequency(commands);
    let most_used = frequencies.first().map(|f| (f.command.clone(), f.count)).unwrap_or_default();

    let time_patterns = analyze_time_patterns(commands);
    let busiest_hour = time_patterns.busiest_hour();
    let busiest_day = time_patterns.busiest_day();

    let mut timestamps: Vec<DateTime<Utc>> = commands.iter().map(|c| c.timestamp).collect();
    timestamps.sort();

    let first_date = timestamps.first().copied();
    let last_date = timestamps.last().copied();

    let total_days = if let (Some(first), Some(last)) = (first_date, last_date) {
        (last - first).num_days().max(1)
    } else {
        1
    };

    let unique_commands: std::collections::HashSet<String> = commands
        .iter()
        .map(|c| c.command_text.clone())
        .collect();

    Insights {
        total_commands: commands.len(),
        unique_commands: unique_commands.len(),
        most_used_command: most_used.0,
        most_used_count: most_used.1,
        busiest_hour,
        busiest_day,
        total_days,
        first_command_date: first_date,
        last_command_date: last_date,
    }
}

