use crate::commandLog::CommandLog;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create the database at the given path
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let path = path.as_ref();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create database directory: {}", parent.display()))?;
        }

        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database: {}", path.display()))?;

        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    /// Initialize database schema
    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                command_text TEXT NOT NULL,
                working_directory TEXT NOT NULL,
                exit_code INTEGER,
                session_id TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                start_time TEXT NOT NULL,
                end_time TEXT,
                total_commands INTEGER DEFAULT 0
            )",
            [],
        )?;

        // Create indexes for better query performance
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON commands(timestamp)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_command_text ON commands(command_text)",
            [],
        )?;

        Ok(())
    }

    /// Insert a command into the database
    pub fn insert_command(&self, command: &CommandLog) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO commands (timestamp, command_text, working_directory, exit_code, session_id)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                command.timestamp.to_rfc3339(),
                command.command_text,
                command.working_directory,
                command.exit_code,
                command.session_id,
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Insert multiple commands in a transaction
    pub fn insert_commands(&mut self, commands: &[CommandLog]) -> Result<usize> {
        let tx = self.conn.transaction()?;
        let mut count = 0;

        for command in commands {
            tx.execute(
                "INSERT OR IGNORE INTO commands (timestamp, command_text, working_directory, exit_code, session_id)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    command.timestamp.to_rfc3339(),
                    command.command_text,
                    command.working_directory,
                    command.exit_code,
                    command.session_id,
                ],
            )?;
            count += 1;
        }

        tx.commit()?;
        Ok(count)
    }

    /// Get total command count
    pub fn total_commands(&self) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM commands",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Get commands within a time range
    pub fn commands_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<CommandLog>> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, command_text, working_directory, exit_code, session_id
             FROM commands
             WHERE timestamp >= ?1 AND timestamp <= ?2
             ORDER BY timestamp ASC",
        )?;

        let commands = stmt.query_map(
            params![start.to_rfc3339(), end.to_rfc3339()],
            |row| {
                let timestamp_str: String = row.get(0)?;
                let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                Ok(CommandLog {
                    timestamp,
                    command_text: row.get(1)?,
                    working_directory: row.get(2)?,
                    exit_code: row.get(3)?,
                    session_id: row.get(4)?,
                })
            },
        )?;

        let mut result = Vec::new();
        for cmd in commands {
            result.push(cmd?);
        }

        Ok(result)
    }

    /// Get all commands
    pub fn all_commands(&self) -> Result<Vec<CommandLog>> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, command_text, working_directory, exit_code, session_id
             FROM commands
             ORDER BY timestamp ASC",
        )?;

        let commands = stmt.query_map([], |row| {
            let timestamp_str: String = row.get(0)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(CommandLog {
                timestamp,
                command_text: row.get(1)?,
                working_directory: row.get(2)?,
                exit_code: row.get(3)?,
                session_id: row.get(4)?,
            })
        })?;

        let mut result = Vec::new();
        for cmd in commands {
            result.push(cmd?);
        }

        Ok(result)
    }
}

impl Default for Database {
    fn default() -> Self {
        let db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rusty-history")
            .join("history.db");

        Self::open(db_path).expect("Failed to create default database")
    }
}

