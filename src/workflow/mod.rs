pub mod runtime;

use crate::config::{WorkflowConfig, WorkflowSourceConfig};
use anyhow::{Context, Result};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowPackageManifest {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub site: String,
    pub display_name: String,
    pub version: String,
    #[serde(default)]
    pub runtime: RuntimeRequirements,
    #[serde(default)]
    pub package: PackageMetadata,
    #[serde(default)]
    pub auth: AuthMetadata,
    #[serde(default)]
    pub ui: UiMetadata,
    #[serde(default)]
    pub commands: CommandIndex,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeRequirements {
    #[serde(default = "default_workflow_api")]
    pub workflow_api: String,
    #[serde(default)]
    pub min_hub_version: Option<String>,
    #[serde(default)]
    pub min_agent_browser_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    #[serde(default = "default_package_kind")]
    pub kind: String,
    #[serde(default = "default_true")]
    pub default_enabled: bool,
}

impl Default for PackageMetadata {
    fn default() -> Self {
        Self {
            kind: default_package_kind(),
            default_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthMetadata {
    #[serde(default = "default_public_strategy")]
    pub strategy: String,
    #[serde(default)]
    pub login_required: bool,
    #[serde(default)]
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiMetadata {
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandIndex {
    #[serde(default)]
    pub include: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowCommandManifest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_public_strategy")]
    pub strategy: String,
    #[serde(default)]
    pub params: Vec<WorkflowParam>,
    #[serde(default)]
    pub execution: ExecutionMetadata,
    #[serde(default)]
    pub output: OutputMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowParam {
    pub name: String,
    #[serde(rename = "type", default = "default_string_type")]
    pub type_: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub positional: bool,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    #[serde(default = "default_script_entry")]
    pub entry: String,
    #[serde(default)]
    pub pipeline: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub native: Option<String>,
}

impl Default for ExecutionMetadata {
    fn default() -> Self {
        Self {
            entry: default_script_entry(),
            pipeline: None,
            script: None,
            native: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputMetadata {
    #[serde(default)]
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowOriginKind {
    Builtin,
    Path,
    Git,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOrigin {
    pub kind: WorkflowOriginKind,
    pub location: String,
    #[serde(rename = "fallbackActive")]
    pub fallback_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPackage {
    pub root_dir: PathBuf,
    pub manifest: WorkflowPackageManifest,
    pub commands: BTreeMap<String, WorkflowCommandManifest>,
    pub origin: WorkflowOrigin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSourceStatus {
    pub site: String,
    pub configured: WorkflowSourceConfig,
    pub mode: String,
    pub fallback_to_builtin: bool,
    pub builtin_available: bool,
    pub resolved: bool,
    pub effective_origin: Option<WorkflowOrigin>,
    pub package_version: Option<String>,
    pub package_display_name: Option<String>,
    pub command_count: usize,
    pub error: Option<String>,
}

impl WorkflowPackage {
    pub fn load_from_dir(root_dir: impl AsRef<Path>, origin: WorkflowOrigin) -> Result<Self> {
        let root_dir = root_dir.as_ref().to_path_buf();
        let manifest_path = root_dir.join("workflow.toml");
        let manifest_str = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read {}", manifest_path.display()))?;
        let manifest: WorkflowPackageManifest = toml::from_str(&manifest_str)
            .with_context(|| format!("failed to parse {}", manifest_path.display()))?;

        let commands_dir = root_dir.join("commands");
        let mut commands = BTreeMap::new();
        if commands_dir.exists() {
            for entry in fs::read_dir(&commands_dir)
                .with_context(|| format!("failed to read {}", commands_dir.display()))?
            {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
                    continue;
                }
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("failed to read {}", path.display()))?;
                let command: WorkflowCommandManifest = toml::from_str(&content)
                    .with_context(|| format!("failed to parse {}", path.display()))?;
                commands.insert(command.name.clone(), command);
            }
        }

        Ok(Self {
            root_dir,
            manifest,
            commands,
            origin,
        })
    }
}

pub fn discover_builtin_packages(workflows_dir: impl AsRef<Path>) -> Result<Vec<WorkflowPackage>> {
    discover_packages_in_dir(workflows_dir, WorkflowOriginKind::Builtin)
}

pub fn discover_packages_in_dir(
    workflows_dir: impl AsRef<Path>,
    origin_kind: WorkflowOriginKind,
) -> Result<Vec<WorkflowPackage>> {
    let workflows_dir = workflows_dir.as_ref();
    if !workflows_dir.exists() {
        return Ok(Vec::new());
    }

    let mut packages = Vec::new();
    for entry in fs::read_dir(workflows_dir)
        .with_context(|| format!("failed to read {}", workflows_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() || !path.join("workflow.toml").exists() {
            continue;
        }

        let origin = WorkflowOrigin {
            kind: origin_kind.clone(),
            location: path.display().to_string(),
            fallback_active: false,
        };
        packages.push(WorkflowPackage::load_from_dir(&path, origin)?);
    }

    packages.sort_by(|a, b| a.manifest.site.cmp(&b.manifest.site));
    Ok(packages)
}

pub fn load_external_package(
    site: &str,
    source: &WorkflowSourceConfig,
    workflow_config: &WorkflowConfig,
) -> Result<WorkflowPackage> {
    match source.source_type.as_str() {
        "path" => load_external_path_package(source),
        "git" => load_external_git_package(site, source, workflow_config),
        other => anyhow::bail!(
            "unsupported workflow source type '{}': site={}",
            other,
            site
        ),
    }
}

pub fn inspect_external_sources(
    workflow_config: &WorkflowConfig,
    builtin_root: impl AsRef<Path>,
) -> Vec<WorkflowSourceStatus> {
    let builtin_root = builtin_root.as_ref();

    workflow_config
        .sources
        .iter()
        .map(|(site, source)| {
            let builtin_available = builtin_root.join(site).join("workflow.toml").exists();
            match inspect_external_package(site, source, workflow_config) {
                Ok(package) => WorkflowSourceStatus {
                    site: site.clone(),
                    configured: source.clone(),
                    mode: workflow_config.mode.clone(),
                    fallback_to_builtin: workflow_config.fallback_to_builtin,
                    builtin_available,
                    resolved: true,
                    effective_origin: Some(package.origin),
                    package_version: Some(package.manifest.version),
                    package_display_name: Some(package.manifest.display_name),
                    command_count: package.commands.len(),
                    error: None,
                },
                Err(error) => WorkflowSourceStatus {
                    site: site.clone(),
                    configured: source.clone(),
                    mode: workflow_config.mode.clone(),
                    fallback_to_builtin: workflow_config.fallback_to_builtin,
                    builtin_available,
                    resolved: false,
                    effective_origin: if builtin_available {
                        Some(WorkflowOrigin {
                            kind: WorkflowOriginKind::Builtin,
                            location: builtin_root.join(site).display().to_string(),
                            fallback_active: workflow_config.fallback_to_builtin,
                        })
                    } else {
                        None
                    },
                    package_version: None,
                    package_display_name: None,
                    command_count: 0,
                    error: Some(error.to_string()),
                },
            }
        })
        .collect()
}

fn inspect_external_package(
    site: &str,
    source: &WorkflowSourceConfig,
    workflow_config: &WorkflowConfig,
) -> Result<WorkflowPackage> {
    match source.source_type.as_str() {
        "path" => load_external_path_package(source),
        "git" => inspect_external_git_package(site, source, workflow_config),
        other => anyhow::bail!(
            "unsupported workflow source type '{}': site={}",
            other,
            site
        ),
    }
}

fn load_external_path_package(source: &WorkflowSourceConfig) -> Result<WorkflowPackage> {
    let path = source
        .path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("workflow path source is missing path"))?;
    let origin = WorkflowOrigin {
        kind: WorkflowOriginKind::Path,
        location: path.clone(),
        fallback_active: false,
    };
    WorkflowPackage::load_from_dir(path, origin)
}

fn load_external_git_package(
    site: &str,
    source: &WorkflowSourceConfig,
    workflow_config: &WorkflowConfig,
) -> Result<WorkflowPackage> {
    let url = source
        .url
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("workflow git source is missing url"))?;
    let repo_dir = prepare_git_checkout(site, source, workflow_config)?;
    let origin = WorkflowOrigin {
        kind: WorkflowOriginKind::Git,
        location: url.clone(),
        fallback_active: false,
    };
    WorkflowPackage::load_from_dir(repo_dir, origin)
}

fn inspect_external_git_package(
    site: &str,
    source: &WorkflowSourceConfig,
    workflow_config: &WorkflowConfig,
) -> Result<WorkflowPackage> {
    let url = source
        .url
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("workflow git source is missing url"))?;
    let repo_dir = workflow_git_cache_dir(site, source, workflow_config, false)?;
    if !repo_dir.exists() {
        anyhow::bail!("workflow git cache is missing for {} from {}", site, url);
    }
    let origin = WorkflowOrigin {
        kind: WorkflowOriginKind::Git,
        location: url.clone(),
        fallback_active: false,
    };
    WorkflowPackage::load_from_dir(repo_dir, origin)
}

fn prepare_git_checkout(
    site: &str,
    source: &WorkflowSourceConfig,
    workflow_config: &WorkflowConfig,
) -> Result<PathBuf> {
    let url = source
        .url
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("workflow git source is missing url"))?;
    let repo_dir = workflow_git_cache_dir(site, source, workflow_config, true)?;

    if !repo_dir.exists() {
        run_git(&[
            "clone",
            "--depth",
            "1",
            url,
            repo_dir
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("invalid workflow cache path"))?,
        ])
        .with_context(|| format!("failed to clone workflow repo for {} from {}", site, url))?;
    }

    if let Some(reference) = source.r#ref.as_ref() {
        run_git_in_repo(&repo_dir, &["fetch", "--depth", "1", "origin", reference]).with_context(
            || {
                format!(
                    "failed to fetch workflow ref '{}' for {} from {}",
                    reference, site, url
                )
            },
        )?;
        run_git_in_repo(&repo_dir, &["checkout", "--force", "FETCH_HEAD"]).with_context(|| {
            format!(
                "failed to checkout workflow ref '{}' for {} from {}",
                reference, site, url
            )
        })?;
    }

    Ok(repo_dir)
}

fn workflow_git_cache_dir(
    site: &str,
    source: &WorkflowSourceConfig,
    workflow_config: &WorkflowConfig,
    create_cache_root: bool,
) -> Result<PathBuf> {
    let url = source
        .url
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("workflow git source is missing url"))?;
    let cache_root = PathBuf::from(&workflow_config.cache_dir);
    if create_cache_root {
        fs::create_dir_all(&cache_root).with_context(|| {
            format!(
                "failed to create workflow cache dir {}",
                cache_root.display()
            )
        })?;
    }

    let mut hasher = Md5::new();
    hasher.update(url.as_bytes());
    if let Some(reference) = source.r#ref.as_ref() {
        hasher.update(reference.as_bytes());
    }
    let cache_key = format!("{:x}", hasher.finalize());
    Ok(cache_root.join(format!("{}-{}", site, cache_key)))
}

fn run_git(args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("failed to launch git with args {:?}", args))?;
    if !output.status.success() {
        anyhow::bail!(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(())
}

fn run_git_in_repo(repo_dir: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .args(args)
        .output()
        .with_context(|| format!("failed to launch git in {}", repo_dir.display()))?;
    if !output.status.success() {
        anyhow::bail!(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(())
}

fn default_schema_version() -> u32 {
    1
}

fn default_workflow_api() -> String {
    "1".to_string()
}

fn default_package_kind() -> String {
    "standard".to_string()
}

fn default_public_strategy() -> String {
    "PUBLIC".to_string()
}

fn default_string_type() -> String {
    "string".to_string()
}

fn default_script_entry() -> String {
    "script".to_string()
}

fn default_true() -> bool {
    true
}
