# Rusty History 🦀

A terminal command analytics tool inspired by Spotify Wrapped. Record, analyze, and display insights about your most-used Linux terminal commands.

## Features

- 📊 **Command Tracking**: Import commands from bash/zsh history files
- 🔍 **Analytics**: Analyze command frequency, time patterns, and usage statistics
- 🎵 **Wrapped Reports**: Generate beautiful "Wrapped" style reports showing your terminal habits
- 💾 **SQLite Storage**: Efficient local database storage
- 🎨 **Beautiful CLI**: Color-coded terminal output with progress bars

## Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd rusty-history

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

## Usage

### Sync Commands from History

Import commands from your shell history files:

```bash
# Auto-detect and import from common history file locations
rusty-history sync

# Import from a specific file
rusty-history sync --file ~/.bash_history
```

### Generate Wrapped Report

Create a beautiful "Wrapped" style report:

```bash
# Current year (default)
rusty-history wrapped

# Specific year
rusty-history wrapped --year 2024
```

### View Statistics

```bash
# Show top 10 commands (default)
rusty-history stats

# Show top 20 commands
rusty-history stats --top 20
```

### View Top Commands

```bash
# Show top 10 commands
rusty-history top

# Show top 5 commands
rusty-history top --count 5
```

### Export Data

Export your command data as JSON:

```bash
# Default: rusty-history-export.json
rusty-history export

# Custom output file
rusty-history export --output my-commands.json
```

## Architecture

The application is organized into several modules:

- **`capture/`**: Parses shell history files (bash, zsh)
- **`storage/`**: SQLite database for command persistence
- **`analysis/`**: Statistical analysis and pattern detection
- **`display/`**: Formatted output and "Wrapped" reports
- **`config/`**: Configuration management

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

## Database Location

By default, the database is stored at:
- Linux: `~/.local/share/rusty-history/history.db`
- Config: `~/.config/rusty-history/config.toml`

## Supported Shells

- **Bash**: `.bash_history` (standard and extended format)
- **Zsh**: `.zsh_history` (extended history format)

## Dependencies

- `clap` - CLI argument parsing
- `rusqlite` - SQLite database
- `chrono` - Date/time handling
- `serde` - Serialization
- `colored` - Terminal colors
- `comfy-table` - Table formatting
- `indicatif` - Progress bars

## Future Enhancements

- Real-time command capture via shell hooks
- Web dashboard
- Command suggestions based on patterns
- Multi-user support
- Integration with tmux/screen

## License

[Your License Here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

