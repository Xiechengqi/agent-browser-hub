use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub password: String,
    pub vnc_url: String,
    pub vnc_username: Option<String>,
    pub vnc_password: Option<String>,
    #[serde(default)]
    pub workflow: WorkflowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    #[serde(default = "default_workflow_mode")]
    pub mode: String,
    #[serde(default = "default_fallback_to_builtin")]
    pub fallback_to_builtin: bool,
    #[serde(default = "default_workflow_cache_dir")]
    pub cache_dir: String,
    #[serde(default)]
    pub sources: BTreeMap<String, WorkflowSourceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSourceConfig {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub r#ref: Option<String>,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            mode: default_workflow_mode(),
            fallback_to_builtin: default_fallback_to_builtin(),
            cache_dir: default_workflow_cache_dir(),
            sources: BTreeMap::new(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            password: "admin123".to_string(),
            vnc_url: "http://localhost:6080".to_string(),
            vnc_username: None,
            vnc_password: None,
            workflow: WorkflowConfig::default(),
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

fn default_workflow_mode() -> String {
    "prefer-external".to_string()
}

fn default_fallback_to_builtin() -> bool {
    true
}

fn default_workflow_cache_dir() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".cache/agent-browser-hub/workflows")
        .display()
        .to_string()
}
