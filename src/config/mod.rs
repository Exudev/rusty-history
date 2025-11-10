use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_path: Option<PathBuf>,
    pub shell: Option<String>,
    pub history_files: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        let db_path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rusty-history")
            .join("history.db");

        Self {
            database_path: Some(db_path),
            shell: None,
            history_files: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rusty-history");

        let config_file = config_dir.join("config.toml");

        if config_file.exists() {
            let content = std::fs::read_to_string(&config_file)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rusty-history");

        std::fs::create_dir_all(&config_dir)?;

        let config_file = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_file, content)?;

        Ok(())
    }

    pub fn database_path(&self) -> PathBuf {
        self.database_path
            .clone()
            .unwrap_or_else(|| {
                dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("rusty-history")
                    .join("history.db")
            })
    }
}

