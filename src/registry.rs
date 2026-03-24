use crate::workflow::{self, WorkflowOrigin};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

pub type NativeCmdFn = fn(&serde_json::Value) -> Result<serde_json::Value>;

#[derive(Clone)]
pub enum CommandSource {
    Native(String),
    Workflow {
        package_root: PathBuf,
        command_file: PathBuf,
        origin: WorkflowOrigin,
    },
}

#[derive(Clone)]
pub struct CommandEntry {
    pub site: String,
    pub name: String,
    pub description: String,
    pub strategy: Option<String>,
    pub source_label: String,
    pub source: CommandSource,
}

#[derive(Clone)]
pub enum ResolvedCommand {
    Pipeline(crate::core::Script),
    WorkflowScript(WorkflowScriptTarget),
    WorkflowNative(WorkflowNativeTarget),
    Native(String),
}

#[derive(Clone)]
pub struct WorkflowScriptTarget {
    pub site: String,
    pub name: String,
    pub package_root: PathBuf,
    pub command_file: PathBuf,
    pub script_path: PathBuf,
}

#[derive(Clone)]
pub struct WorkflowNativeTarget {
    pub site: String,
    pub name: String,
    pub handler: String,
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

    pub fn discover_workflow_packages(&mut self, workflows_dir: &str) -> Result<()> {
        for package in workflow::discover_builtin_packages(workflows_dir)? {
            self.register_workflow_package(package, "workflow:builtin")?;
        }
        Ok(())
    }

    pub fn discover_external_workflow_packages(
        &mut self,
        config: &crate::config::WorkflowConfig,
    ) -> Result<()> {
        if config.mode == "builtin-only" {
            return Ok(());
        }

        for (site, source) in &config.sources {
            match workflow::load_external_package(site, source, config) {
                Ok(package) => {
                    if package.manifest.site != *site {
                        anyhow::bail!(
                            "workflow source site mismatch: expected {}, got {}",
                            site,
                            package.manifest.site
                        );
                    }
                    self.remove_site(site);
                    self.register_workflow_package(package, "workflow:external")?;
                }
                Err(err) => {
                    if config.mode == "strict-external" && !config.fallback_to_builtin {
                        return Err(err);
                    }
                    eprintln!(
                        "[workflow] ignoring external workflow package for {}: {}",
                        site, err
                    );
                }
            }
        }

        Ok(())
    }

    fn register_workflow_package(
        &mut self,
        package: crate::workflow::WorkflowPackage,
        source_label: &str,
    ) -> Result<()> {
        for (name, command) in package.commands {
            let command_file = package
                .root_dir
                .join("commands")
                .join(format!("{}.toml", name));
            self.register(CommandEntry {
                site: package.manifest.site.clone(),
                name: name.clone(),
                description: command.description,
                strategy: Some(command.strategy),
                source_label: source_label.to_string(),
                source: CommandSource::Workflow {
                    package_root: package.root_dir.clone(),
                    command_file,
                    origin: package.origin.clone(),
                },
            });
        }
        Ok(())
    }

    fn remove_site(&mut self, site: &str) {
        let stale_keys: Vec<String> = self
            .commands
            .keys()
            .filter(|key| key.starts_with(&format!("{}/", site)))
            .cloned()
            .collect();
        for key in stale_keys {
            self.commands.remove(&key);
        }
    }
}

pub fn build_default_registry() -> Result<Registry> {
    let mut registry = Registry::new();
    let config = crate::config::load_config();
    crate::commands::register_all(&mut registry);
    registry.discover_workflow_packages("workflows")?;
    registry.discover_external_workflow_packages(&config.workflow)?;
    Ok(registry)
}

pub fn resolve_command_entry(entry: &CommandEntry) -> Result<ResolvedCommand> {
    match &entry.source {
        CommandSource::Workflow {
            package_root,
            command_file,
            ..
        } => {
            let content = std::fs::read_to_string(command_file)?;
            let command: crate::workflow::WorkflowCommandManifest = toml::from_str(&content)?;
            let workflow_manifest = load_workflow_manifest(package_root)?;
            match command.execution.entry.as_str() {
                "pipeline" => resolve_workflow_asset(
                    entry,
                    package_root,
                    &workflow_manifest,
                    &command,
                    command.execution.pipeline.as_deref(),
                    "pipeline",
                ),
                "script" => {
                    let script_rel = command.execution.script.as_deref().ok_or_else(|| {
                        anyhow::anyhow!(
                            "workflow script path is missing for {}/{}",
                            entry.site,
                            entry.name
                        )
                    })?;
                    if is_yaml_asset(script_rel) {
                        resolve_workflow_asset(
                            entry,
                            package_root,
                            &workflow_manifest,
                            &command,
                            Some(script_rel),
                            "script",
                        )
                    } else {
                        Ok(ResolvedCommand::WorkflowScript(WorkflowScriptTarget {
                            site: entry.site.clone(),
                            name: entry.name.clone(),
                            package_root: package_root.clone(),
                            command_file: command_file.clone(),
                            script_path: package_root.join(script_rel),
                        }))
                    }
                }
                "native" => {
                    let handler = command.execution.native.clone().ok_or_else(|| {
                        anyhow::anyhow!(
                            "workflow native handler is missing for {}/{}",
                            entry.site,
                            entry.name
                        )
                    })?;
                    Ok(ResolvedCommand::WorkflowNative(WorkflowNativeTarget {
                        site: entry.site.clone(),
                        name: entry.name.clone(),
                        handler,
                    }))
                }
                other => anyhow::bail!(
                    "workflow execution entry '{}' is not supported yet for {}/{}",
                    other,
                    entry.site,
                    entry.name
                ),
            }
        }
        CommandSource::Native(name) => Ok(ResolvedCommand::Native(name.clone())),
    }
}

pub fn load_script_from_entry(entry: &CommandEntry) -> Result<crate::core::Script> {
    match resolve_command_entry(entry)? {
        ResolvedCommand::Pipeline(script) => Ok(script),
        ResolvedCommand::WorkflowScript(target) => {
            crate::workflow::runtime::load_workflow_script_definition(&target)
        }
        ResolvedCommand::WorkflowNative(target) => anyhow::bail!(
            "workflow native execution backend is not implemented yet for {}/{} ({})",
            target.site,
            target.name,
            target.handler
        ),
        ResolvedCommand::Native(name) => anyhow::bail!(
            "native command execution is not routed through script loader yet: {}",
            name
        ),
    }
}

fn resolve_workflow_asset(
    entry: &CommandEntry,
    package_root: &std::path::Path,
    workflow_manifest: &crate::workflow::WorkflowPackageManifest,
    command: &crate::workflow::WorkflowCommandManifest,
    asset_rel: Option<&str>,
    entry_kind: &str,
) -> Result<ResolvedCommand> {
    let asset_rel = asset_rel.ok_or_else(|| {
        anyhow::anyhow!(
            "workflow {} path is missing for {}/{}",
            entry_kind,
            entry.site,
            entry.name
        )
    })?;
    let asset_path = package_root.join(asset_rel);
    let asset_str = std::fs::read_to_string(&asset_path)?;

    if let Ok(pipeline) = serde_yaml::from_str::<Vec<serde_json::Value>>(&asset_str) {
        return Ok(ResolvedCommand::Pipeline(build_pipeline_script(
            entry,
            workflow_manifest,
            command,
            pipeline,
        )?));
    }

    let mut script: crate::core::Script = serde_yaml::from_str(&asset_str)?;
    apply_workflow_defaults(entry, workflow_manifest, command, &mut script)?;
    Ok(ResolvedCommand::Pipeline(script))
}

fn is_yaml_asset(path: &str) -> bool {
    path.ends_with(".yaml") || path.ends_with(".yml")
}

fn build_pipeline_script(
    entry: &CommandEntry,
    workflow_manifest: &crate::workflow::WorkflowPackageManifest,
    command: &crate::workflow::WorkflowCommandManifest,
    pipeline: Vec<serde_json::Value>,
) -> Result<crate::core::Script> {
    let args_obj = build_args_object(command);
    let strategy: crate::core::Strategy =
        serde_json::from_value(serde_json::Value::String(command.strategy.clone()))?;
    let resolved_domain = resolve_domain(entry, workflow_manifest);

    Ok(crate::core::Script {
        meta: None,
        config: None,
        params: vec![],
        steps: vec![],
        site: Some(entry.site.clone()),
        name: Some(entry.name.clone()),
        domain: Some(resolved_domain),
        strategy: Some(strategy),
        browser: Some(command.strategy != "PUBLIC"),
        args: Some(serde_json::Value::Object(args_obj)),
        pipeline: Some(pipeline),
    })
}

fn apply_workflow_defaults(
    entry: &CommandEntry,
    workflow_manifest: &crate::workflow::WorkflowPackageManifest,
    command: &crate::workflow::WorkflowCommandManifest,
    script: &mut crate::core::Script,
) -> Result<()> {
    if script.site.is_none() {
        script.site = Some(entry.site.clone());
    }
    if script.name.is_none() {
        script.name = Some(entry.name.clone());
    }
    if script.domain.is_none() {
        script.domain = Some(resolve_domain(entry, workflow_manifest));
    }
    if script.strategy.is_none() {
        let strategy: crate::core::Strategy =
            serde_json::from_value(serde_json::Value::String(command.strategy.clone()))?;
        script.strategy = Some(strategy);
    }
    if script.browser.is_none() {
        script.browser = Some(command.strategy != "PUBLIC");
    }
    if script.args.is_none() && !command.params.is_empty() {
        script.args = Some(serde_json::Value::Object(build_args_object(command)));
    }
    Ok(())
}

fn build_args_object(
    command: &crate::workflow::WorkflowCommandManifest,
) -> serde_json::Map<String, serde_json::Value> {
    command
        .params
        .iter()
        .map(|param| {
            (
                param.name.clone(),
                serde_json::json!({
                    "type": param.type_.clone(),
                    "required": param.required,
                    "default": param.default.clone(),
                }),
            )
        })
        .collect()
}

fn resolve_domain(
    entry: &CommandEntry,
    workflow_manifest: &crate::workflow::WorkflowPackageManifest,
) -> String {
    workflow_manifest
        .auth
        .domains
        .first()
        .cloned()
        .unwrap_or_else(|| entry.site.clone())
}

fn load_workflow_manifest(
    package_root: &std::path::Path,
) -> Result<crate::workflow::WorkflowPackageManifest> {
    let workflow_manifest_path = package_root.join("workflow.toml");
    let workflow_manifest_str = std::fs::read_to_string(&workflow_manifest_path)?;
    let workflow_manifest: crate::workflow::WorkflowPackageManifest =
        toml::from_str(&workflow_manifest_str)?;
    Ok(workflow_manifest)
}
