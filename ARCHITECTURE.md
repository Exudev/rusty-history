# Rusty History - Architecture Design

## Overview
Rusty History is a terminal command analytics tool inspired by Spotify Wrapped. It records, analyzes, and displays insights about your most-used Linux terminal commands.

## Core Architecture

### 1. **Command Capture Module** (`src/capture/`)
   - **Purpose**: Extract commands from shell history files
   - **Responsibilities**:
     - Parse `.bash_history`, `.zsh_history`, and other shell history files
     - Handle different history formats (with/without timestamps)
     - Support real-time capture via shell hooks (future enhancement)
   - **Key Functions**:
     - `parse_bash_history()` - Parse bash history format
     - `parse_zsh_history()` - Parse zsh extended history format
     - `detect_shell()` - Auto-detect user's shell
     - `read_history_file()` - Read and parse history files

### 2. **Storage Module** (`src/storage/`)
   - **Purpose**: Persist command data efficiently
   - **Responsibilities**:
     - Store commands in SQLite database
     - Track metadata: timestamp, working directory, command text, exit code
     - Handle database migrations and schema updates
   - **Database Schema**:
     - `commands` table: id, timestamp, command_text, working_directory, exit_code, session_id
     - `sessions` table: id, start_time, end_time, total_commands
     - `statistics` table: date, total_commands, unique_commands, most_used_command

### 3. **Analysis Module** (`src/analysis/`)
   - **Purpose**: Process and analyze command data
   - **Responsibilities**:
     - Calculate frequency statistics
     - Identify patterns (time-based, directory-based)
     - Generate insights (most used commands, longest sessions, etc.)
     - Time-based aggregations (daily, weekly, monthly, yearly)
   - **Key Functions**:
     - `calculate_frequencies()` - Count command usage
     - `analyze_time_patterns()` - Commands by time of day
     - `analyze_directory_patterns()` - Commands by working directory
     - `generate_insights()` - Create "Wrapped" style insights

### 4. **Display Module** (`src/display/`)
   - **Purpose**: Format and present analytics
   - **Responsibilities**:
     - Generate formatted CLI output
     - Create "Wrapped" style reports
     - Export to JSON/CSV
     - Color-coded terminal output
   - **Key Functions**:
     - `display_wrapped_report()` - Main "Wrapped" style output
     - `display_top_commands()` - Show most used commands
     - `display_time_stats()` - Time-based statistics
     - `export_json()` - Export data as JSON

### 5. **Configuration Module** (`src/config/`)
   - **Purpose**: Manage user preferences and settings
   - **Responsibilities**:
     - Load/save configuration file
     - Shell detection and configuration
     - Database path configuration
     - Display preferences
   - **Config File** (`~/.config/rusty-history/config.toml`):
     - Shell paths
     - History file locations
     - Database path
     - Analysis preferences

### 6. **Main Module** (`src/main.rs`)
   - **Purpose**: CLI interface and orchestration
   - **Responsibilities**:
     - Parse command-line arguments
     - Coordinate between modules
     - Handle errors gracefully
   - **CLI Commands**:
     - `rusty-history sync` - Import commands from history files
     - `rusty-history wrapped` - Generate and display wrapped report
     - `rusty-history stats` - Show general statistics
     - `rusty-history top` - Show top N commands
     - `rusty-history export` - Export data

## Recommended Crates

### Core Dependencies
- **`clap`** - CLI argument parsing
- **`serde`** + **`serde_json`** - Serialization/deserialization
- **`chrono`** - Date/time handling
- **`rusqlite`** - SQLite database
- **`dirs`** - Standard directory paths (home, config, etc.)
- **`anyhow`** - Error handling
- **`thiserror`** - Custom error types

### Display & Formatting
- **`colored`** or **`owo-colors`** - Terminal colors
- **`tabled`** or **`comfy-table`** - Table formatting
- **`indicatif`** - Progress bars

### Optional/Advanced
- **`regex`** - Pattern matching for command parsing
- **`rayon`** - Parallel processing for large history files
- **`tokio`** - Async runtime (if real-time capture is needed)

## Data Flow

```
Shell History Files → Capture Module → Storage Module (SQLite)
                                                      ↓
Display Module ← Analysis Module ← Storage Module
```

## File Structure

```
src/
├── main.rs              # CLI entry point
├── commandLog.rs        # Data structures (existing)
├── capture/
│   ├── mod.rs
│   ├── bash.rs
│   └── zsh.rs
├── storage/
│   ├── mod.rs
│   └── database.rs
├── analysis/
│   ├── mod.rs
│   ├── frequency.rs
│   └── patterns.rs
├── display/
│   ├── mod.rs
│   ├── wrapped.rs
│   └── stats.rs
└── config/
    ├── mod.rs
    └── settings.rs
```

## Future Enhancements
- Real-time command capture via shell hooks
- Web dashboard
- Command suggestions based on patterns
- Integration with other tools (tmux, screen)
- Multi-user support
- Command aliasing and grouping

