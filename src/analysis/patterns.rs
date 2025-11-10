use crate::commandLog::CommandLog;
use chrono::{Datelike, Timelike, Weekday};
use std::collections::HashMap;

/// Time-based pattern analysis
#[derive(Debug)]
pub struct TimePatterns {
    hour_counts: HashMap<u8, u64>,
    day_counts: HashMap<Weekday, u64>,
}

impl TimePatterns {
    pub fn new() -> Self {
        Self {
            hour_counts: HashMap::new(),
            day_counts: HashMap::new(),
        }
    }

    pub fn busiest_hour(&self) -> u8 {
        self.hour_counts
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&hour, _)| hour)
            .unwrap_or(12)
    }

    pub fn busiest_day(&self) -> String {
        self.day_counts
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&day, _)| format!("{:?}", day))
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn hour_distribution(&self) -> &HashMap<u8, u64> {
        &self.hour_counts
    }

    pub fn day_distribution(&self) -> &HashMap<Weekday, u64> {
        &self.day_counts
    }
}

pub fn analyze_time_patterns(commands: &[CommandLog]) -> TimePatterns {
    let mut patterns = TimePatterns::new();

    for cmd in commands {
        let hour = cmd.timestamp.hour() as u8;
        let weekday = cmd.timestamp.weekday();

        *patterns.hour_counts.entry(hour).or_insert(0) += 1;
        *patterns.day_counts.entry(weekday).or_insert(0) += 1;
    }

    patterns
}

