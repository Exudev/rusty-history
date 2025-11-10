use crate::analysis::CommandFrequency;
use comfy_table::Table;

/// Display statistics in a table format
pub fn display_stats(frequencies: &[CommandFrequency], title: &str) {
    let mut table = Table::new();
    table.set_header(vec!["Rank", "Command", "Count", "Percentage"]);

    for (i, freq) in frequencies.iter().enumerate() {
        table.add_row(vec![
            (i + 1).to_string(),
            freq.command.clone(),
            freq.count.to_string(),
            format!("{:.2}%", freq.percentage),
        ]);
    }

    println!("\n{}", title);
    println!("{}", table);
}

/// Display top commands in a simple list
pub fn display_top_commands(frequencies: &[CommandFrequency], n: usize) {
    println!("\n📊 Top {} Commands:\n", n);
    
    for (i, freq) in frequencies.iter().take(n).enumerate() {
        println!("  {}. {} ({} times, {:.1}%)", 
                 i + 1,
                 freq.command,
                 freq.count,
                 freq.percentage);
    }
    println!();
}

