use crate::analysis::Insights;
use crate::analysis::CommandFrequency;
use colored::*;

/// Display a "Wrapped" style report
pub fn display_wrapped(insights: &Insights, top_commands: &[CommandFrequency]) {
    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "     🎵 YOUR TERMINAL WRAPPED 🎵".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n".bright_cyan());

    // Total commands
    println!("{}", "📊 YOUR YEAR IN COMMANDS".bright_yellow().bold());
    println!("   You ran {} commands this year!", 
             insights.total_commands.to_string().bright_white().bold());
    println!("   {} unique commands", 
             insights.unique_commands.to_string().bright_white());
    
    if let Some(first) = insights.first_command_date {
        if let Some(last) = insights.last_command_date {
            println!("   From {} to {}", 
                     first.format("%B %d, %Y").to_string().bright_white(),
                     last.format("%B %d, %Y").to_string().bright_white());
        }
    }
    println!();

    // Most used command
    if !insights.most_used_command.is_empty() {
        println!("{}", "⭐ YOUR TOP COMMAND".bright_magenta().bold());
        println!("   {} (used {} times)", 
                 format!("`{}`", insights.most_used_command).bright_white().bold(),
                 insights.most_used_count.to_string().bright_white());
        println!();
    }

    // Top commands
    if !top_commands.is_empty() {
        println!("{}", "🏆 TOP 5 COMMANDS".bright_green().bold());
        for (i, cmd) in top_commands.iter().take(5).enumerate() {
            let emoji = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };
            println!("   {} {} {} ({} times, {:.1}%)", 
                     emoji,
                     format!("`{}`", cmd.command).bright_white(),
                     "─".bright_black(),
                     cmd.count.to_string().bright_white(),
                     cmd.percentage);
        }
        println!();
    }

    // Time patterns
    println!("{}", "⏰ YOUR TERMINAL HABITS".bright_blue().bold());
    println!("   Busiest hour: {}:00", 
             insights.busiest_hour.to_string().bright_white().bold());
    println!("   Busiest day: {}", 
             insights.busiest_day.bright_white().bold());
    println!();

    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "   Thanks for using Rusty History! 🦀".bright_cyan());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n".bright_cyan());
}

