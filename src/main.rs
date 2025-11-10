mod analysis;
mod capture;
mod commandLog;
mod config;
mod display;
mod storage;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};

use crate::analysis::{generate_insights, top_commands};
use crate::capture::{find_history_files, parse_history_file};
use crate::config::Config;
use crate::display::{display_stats, display_top_commands, display_wrapped};
use crate::storage::Database;

#[derive(Parser)]
#[command(name = "rusty-history")]
#[command(about = "Record, analyze, and display your most-used Linux terminal commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync commands from shell history files into the database
    Sync {
        /// Path to specific history file (optional)
        #[arg(short, long)]
        file: Option<String>,
    },
    /// Generate and display a "Wrapped" style report
    Wrapped {
        /// Year to analyze (default: current year)
        #[arg(short, long)]
        year: Option<i32>,
    },
    /// Show general statistics
    Stats {
        /// Number of top commands to show
        #[arg(short, long, default_value = "10")]
        top: usize,
    },
    /// Show top N commands
    Top {
        /// Number of commands to show
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    /// Export data as JSON
    Export {
        /// Output file path
        #[arg(short, long, default_value = "rusty-history-export.json")]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sync { file } => sync_commands(file)?,
        Commands::Wrapped { year } => show_wrapped(year)?,
        Commands::Stats { top } => show_stats(top)?,
        Commands::Top { count } => show_top(count)?,
        Commands::Export { output } => export_data(output)?,
    }

    Ok(())
}

fn sync_commands(file: Option<String>) -> Result<()> {
    println!("🔄 Syncing commands from history files...\n");

    let config = Config::load()?;
    let mut db = Database::open(config.database_path())?;

    let history_files = if let Some(file_path) = file {
        vec![std::path::PathBuf::from(file_path)]
    } else {
        let files = find_history_files();
        if files.is_empty() {
            anyhow::bail!("No history files found. Please specify a file with --file");
        }
        files
    };

    let pb = ProgressBar::new(history_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut total_imported = 0;

    for history_file in &history_files {
        pb.set_message(format!("Processing: {}", history_file.display()));

        match parse_history_file(history_file) {
            Ok(commands) => {
                let count = db.insert_commands(&commands)?;
                total_imported += count;
                pb.inc(1);
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to parse {}: {}", history_file.display(), e);
                pb.inc(1);
            }
        }
    }

    pb.finish_with_message("✅ Sync complete!");

    println!("\n📊 Imported {} commands from {} file(s)", total_imported, history_files.len());
    println!("💾 Database location: {}\n", config.database_path().display());

    Ok(())
}

fn show_wrapped(year: Option<i32>) -> Result<()> {
    let config = Config::load()?;
    let db = Database::open(config.database_path())?;

    let all_commands = db.all_commands()?;

    if all_commands.is_empty() {
        println!("❌ No commands found in database. Run 'rusty-history sync' first.");
        return Ok(());
    }

    // Filter by year if specified
    let commands: Vec<_> = if let Some(year) = year {
        all_commands
            .into_iter()
            .filter(|cmd| cmd.timestamp.year() == year)
            .collect()
    } else {
        all_commands
    };

    if commands.is_empty() {
        println!("❌ No commands found for the specified period.");
        return Ok(());
    }

    let insights = generate_insights(&commands);
    let top = top_commands(&commands, 5);

    display_wrapped(&insights, &top);

    Ok(())
}

fn show_stats(top: usize) -> Result<()> {
    let config = Config::load()?;
    let db = Database::open(config.database_path())?;

    let commands = db.all_commands()?;

    if commands.is_empty() {
        println!("❌ No commands found in database. Run 'rusty-history sync' first.");
        return Ok(());
    }

    let frequencies = crate::analysis::analyze_frequency(&commands);
    let top_freqs: Vec<_> = frequencies.into_iter().take(top).collect();

    display_stats(&top_freqs, &format!("📊 Top {} Commands", top));

    Ok(())
}

fn show_top(count: usize) -> Result<()> {
    let config = Config::load()?;
    let db = Database::open(config.database_path())?;

    let commands = db.all_commands()?;

    if commands.is_empty() {
        println!("❌ No commands found in database. Run 'rusty-history sync' first.");
        return Ok(());
    }

    let frequencies = crate::analysis::analyze_frequency(&commands);
    let top_freqs: Vec<_> = frequencies.into_iter().take(count).collect();

    display_top_commands(&top_freqs, count);

    Ok(())
}

fn export_data(output: String) -> Result<()> {
    let config = Config::load()?;
    let db = Database::open(config.database_path())?;

    let commands = db.all_commands()?;

    if commands.is_empty() {
        println!("❌ No commands found in database. Run 'rusty-history sync' first.");
        return Ok(());
    }

    let json = serde_json::to_string_pretty(&commands)
        .context("Failed to serialize commands to JSON")?;

    std::fs::write(&output, json)
        .with_context(|| format!("Failed to write to {}", output))?;

    println!("✅ Exported {} commands to {}\n", commands.len(), output);

    Ok(())
}
