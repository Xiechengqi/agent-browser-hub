use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub enum NativeDispatchTarget {
    Registered {
        handler: String,
    },
    Workflow {
        site: String,
        name: String,
        handler: String,
    },
}

pub async fn execute_native_dispatch(
    target: NativeDispatchTarget,
    params: HashMap<String, Value>,
) -> Result<Value> {
    match target {
        NativeDispatchTarget::Registered { handler } => {
            crate::commands::dispatch_native(&handler, params).await
        }
        NativeDispatchTarget::Workflow { handler, .. } => {
            crate::commands::dispatch_native(&handler, params).await
        }
    }
}

pub async fn execute_workflow_script(
    target: crate::registry::WorkflowScriptTarget,
    params: HashMap<String, Value>,
) -> Result<Value> {
    let script = load_workflow_script_definition(&target)?;
    let executor = crate::core::Executor::new().await?;
    let result = executor.execute(&script, params).await?;
    Ok(result.result)
}

pub fn load_workflow_script_definition(
    target: &crate::registry::WorkflowScriptTarget,
) -> Result<crate::core::Script> {
    let content = std::fs::read_to_string(&target.script_path)?;
    let mut script = match target
        .script_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "yaml" | "yml" => load_yaml_workflow_script(&content)?,
        "json" => load_json_workflow_script(&content)?,
        "toml" => toml::from_str::<crate::core::Script>(&content)?,
        other => anyhow::bail!(
            "unsupported workflow script asset extension '{}' for {}/{} ({})",
            other,
            target.site,
            target.name,
            target.script_path.display()
        ),
    };

    apply_workflow_script_defaults(target, &mut script)?;
    Ok(script)
}

fn apply_workflow_script_defaults(
    target: &crate::registry::WorkflowScriptTarget,
    script: &mut crate::core::Script,
) -> Result<()> {
    let command_content = std::fs::read_to_string(&target.command_file)?;
    let command: crate::workflow::WorkflowCommandManifest = toml::from_str(&command_content)?;

    let workflow_manifest_path = target.package_root.join("workflow.toml");
    let workflow_manifest_content = std::fs::read_to_string(&workflow_manifest_path)?;
    let workflow_manifest: crate::workflow::WorkflowPackageManifest =
        toml::from_str(&workflow_manifest_content)?;

    if script.site.is_none() {
        script.site = Some(target.site.clone());
    }
    if script.name.is_none() {
        script.name = Some(target.name.clone());
    }
    if script.domain.is_none() {
        script.domain = workflow_manifest
            .auth
            .domains
            .first()
            .cloned()
            .or_else(|| Some(target.site.clone()));
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
        let args: serde_json::Map<String, Value> = command
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
            .collect();
        script.args = Some(Value::Object(args));
    }

    Ok(())
}

fn load_yaml_workflow_script(content: &str) -> Result<crate::core::Script> {
    if let Ok(script) = serde_yaml::from_str::<crate::core::Script>(content) {
        return Ok(script);
    }

    let value: serde_yaml::Value = serde_yaml::from_str(content)?;
    if let Some(inner) = value
        .as_mapping()
        .and_then(|mapping| mapping.get(serde_yaml::Value::String("pipeline".to_string())))
    {
        return Ok(serde_yaml::from_value(inner.clone())?);
    }

    anyhow::bail!("unsupported workflow yaml script shape")
}

fn load_json_workflow_script(content: &str) -> Result<crate::core::Script> {
    if let Ok(script) = serde_json::from_str::<crate::core::Script>(content) {
        return Ok(script);
    }

    let value: Value = serde_json::from_str(content)?;
    if let Some(inner) = value.get("pipeline") {
        return Ok(serde_json::from_value(inner.clone())?);
    }

    anyhow::bail!("unsupported workflow json script shape")
}
