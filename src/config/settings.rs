use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Paths to scan for build artifacts in purge command
    #[serde(default = "default_purge_paths")]
    pub purge_paths: Vec<String>,

    /// Paths that should never be cleaned
    #[serde(default)]
    pub whitelist: Vec<String>,

    /// Default categories for clean command
    #[serde(default = "default_categories")]
    pub default_categories: Vec<String>,

    /// Age threshold in days for downloads cleanup
    #[serde(default = "default_download_age")]
    pub download_age_days: u64,

    /// Age threshold in days for purge command
    #[serde(default = "default_purge_age")]
    pub purge_age_days: u64,
}

fn default_purge_paths() -> Vec<String> {
    let home = dirs::home_dir().unwrap_or_default();
    vec![
        home.join("Projects").to_string_lossy().to_string(),
        home.join("Developer").to_string_lossy().to_string(),
        home.join("Documents").to_string_lossy().to_string(),
    ]
}

fn default_categories() -> Vec<String> {
    vec![
        "user-cache".to_string(),
        "system-logs".to_string(),
        "crash-reports".to_string(),
        "browser-cache".to_string(),
        "trash".to_string(),
    ]
}

fn default_download_age() -> u64 {
    30
}

fn default_purge_age() -> u64 {
    7
}

impl Default for Config {
    fn default() -> Self {
        Config {
            purge_paths: default_purge_paths(),
            whitelist: vec![],
            default_categories: default_categories(),
            download_age_days: default_download_age(),
            purge_age_days: default_purge_age(),
        }
    }
}

impl Config {
    /// Load config from file or create default
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    /// Get config file path
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".config")
            .join("mop")
            .join("config.toml")
    }

    /// Check if a path is whitelisted
    pub fn is_whitelisted(&self, path: &str) -> bool {
        self.whitelist.iter().any(|w| path.starts_with(w))
    }
}
