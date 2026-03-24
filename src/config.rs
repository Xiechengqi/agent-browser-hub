use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub password: String,
    pub vnc_url: String,
    pub vnc_username: Option<String>,
    pub vnc_password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            password: "admin123".to_string(),
            vnc_url: "http://localhost:6080".to_string(),
            vnc_username: None,
            vnc_password: None,
        }
    }
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/agent-browser-hub/config.toml")
}

pub fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
    std::fs::write(&path, content)?;
    Ok(())
}
