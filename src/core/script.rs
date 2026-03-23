use serde::{Deserialize, Serialize};
use crate::core::Strategy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    #[serde(default)]
    pub params: Vec<ParamDef>,
    #[serde(default)]
    pub steps: Vec<Step>,

    // opencli format fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Vec<serde_json::Value>>,
}

impl Script {
    pub fn normalize(&self) -> CommandConfig {
        if let Some(ref meta) = self.meta {
            // Format A (agent-browser-hub)
            CommandConfig {
                id: meta.id.clone(),
                name: meta.name.clone(),
                description: meta.description.clone(),
                domain: self.config.as_ref().map(|c| c.domain.clone()).unwrap_or_default(),
                strategy: self.config.as_ref().and_then(|c| c.strategy.clone()).unwrap_or_default(),
                timeout: self.config.as_ref().map(|c| c.timeout).unwrap_or(30),
                params: self.params.clone(),
                steps: self.steps.clone(),
                pipeline: None,
            }
        } else {
            // Format B (opencli)
            let site = self.site.as_ref().map(|s| s.as_str()).unwrap_or("unknown");
            let name = self.name.as_ref().map(|s| s.as_str()).unwrap_or("command");

            CommandConfig {
                id: format!("{}-{}", site, name),
                name: name.to_string(),
                description: String::new(),
                domain: site.to_string(),
                strategy: self.strategy.clone().unwrap_or_default(),
                timeout: 30,
                params: self.extract_params(),
                steps: vec![],
                pipeline: self.pipeline.clone(),
            }
        }
    }

    fn extract_params(&self) -> Vec<ParamDef> {
        if let Some(args_obj) = &self.args {
            if let Some(obj) = args_obj.as_object() {
                return obj.iter().map(|(name, def)| {
                    ParamDef {
                        name: name.clone(),
                        type_: def.get("type").and_then(|v| v.as_str()).unwrap_or("string").to_string(),
                        required: def.get("required").and_then(|v| v.as_bool()).unwrap_or(false),
                        positional: false,
                        default: def.get("default").cloned(),
                    }
                }).collect();
            }
        }
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct CommandConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub domain: String,
    pub strategy: Strategy,
    pub timeout: u64,
    pub params: Vec<ParamDef>,
    pub steps: Vec<Step>,
    pub pipeline: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 { 30 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub positional: bool,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExecutionResult {
    pub execution_id: String,
    pub status: String,
    pub duration_ms: u64,
    pub result: serde_json::Value,
}
