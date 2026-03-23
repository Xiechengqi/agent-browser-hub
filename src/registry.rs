use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

pub type NativeCmdFn = fn(&serde_json::Value) -> Result<serde_json::Value>;

#[derive(Clone)]
pub enum CommandSource {
    Yaml(PathBuf),
    Native(String), // Function name for native commands
}

#[derive(Clone)]
pub struct CommandEntry {
    pub site: String,
    pub name: String,
    pub description: String,
    pub source: CommandSource,
}

pub struct Registry {
    commands: HashMap<String, CommandEntry>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, entry: CommandEntry) {
        let key = format!("{}/{}", entry.site, entry.name);
        self.commands.insert(key, entry);
    }

    pub fn get(&self, site: &str, name: &str) -> Option<&CommandEntry> {
        let key = format!("{}/{}", site, name);
        self.commands.get(&key)
    }

    pub fn list(&self) -> Vec<&CommandEntry> {
        self.commands.values().collect()
    }

    pub fn discover_yaml_scripts(&mut self, scripts_dir: &str) -> Result<()> {
        let path = std::path::Path::new(scripts_dir);
        if !path.exists() {
            return Ok(());
        }

        for site_entry in std::fs::read_dir(path)? {
            let site_entry = site_entry?;
            if !site_entry.path().is_dir() {
                continue;
            }
            let site = site_entry.file_name().to_string_lossy().to_string();

            for file_entry in std::fs::read_dir(site_entry.path())? {
                let file_entry = file_entry?;
                let file_path = file_entry.path();
                if file_path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                    continue;
                }

                let name = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                let description = std::fs::read_to_string(&file_path)
                    .ok()
                    .and_then(|c| serde_yaml::from_str::<serde_json::Value>(&c).ok())
                    .and_then(|v| {
                        v["meta"]["description"]
                            .as_str()
                            .or_else(|| v["description"].as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_default();

                self.register(CommandEntry {
                    site: site.clone(),
                    name,
                    description,
                    source: CommandSource::Yaml(file_path),
                });
            }
        }

        Ok(())
    }
}
