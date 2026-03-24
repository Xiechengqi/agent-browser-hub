use agent_browser_hub::cli::{Cli, Commands};
use agent_browser_hub::server;
use agent_browser_hub::GITHUB_REPO;
use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Serve { port }) => {
            server::start(port).await?;
        }
        Some(Commands::List) => {
            cli_list()?;
        }
        Some(Commands::Version) => {
            println!("agent-browser-hub v{}", VERSION);
        }
        Some(Commands::Upgrade) => {
            cli_upgrade().await?;
        }
        Some(Commands::Run {
            script,
            format,
            params,
        }) => {
            cli_run(&script, &format, params).await?;
        }
        None => {
            println!("agent-browser-hub v{}", VERSION);
            println!("Use --help for usage");
        }
    }

    Ok(())
}

fn cli_list() -> anyhow::Result<()> {
    let registry = agent_browser_hub::registry::build_default_registry()?;

    println!("Available scripts:");
    let commands = registry.list();
    if commands.is_empty() {
        println!("  (no scripts found)");
    } else {
        for cmd in commands {
            let strategy = cmd
                .strategy
                .clone()
                .unwrap_or_else(|| "UNKNOWN".to_string());
            println!(
                "  {}/{:<20} [{}|{}] - {}",
                cmd.site, cmd.name, strategy, cmd.source_label, cmd.description
            );
        }
    }
    Ok(())
}

async fn cli_run(script: &str, format: &str, params: Vec<String>) -> anyhow::Result<()> {
    let parts: Vec<&str> = script.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid script format. Use: site/command");
    }
    let (site, command) = (parts[0], parts[1]);

    let registry = agent_browser_hub::registry::build_default_registry()?;

    let entry = registry
        .get(site, command)
        .ok_or_else(|| anyhow::anyhow!("Command not found: {}/{}", site, command))?;
    let resolved = agent_browser_hub::registry::resolve_command_entry(entry)?;

    let mut param_map = std::collections::HashMap::new();
    let mut i = 0;
    while i < params.len() {
        if params[i].starts_with("--") {
            let key = params[i].trim_start_matches("--");
            if i + 1 < params.len() {
                param_map.insert(
                    key.to_string(),
                    serde_json::Value::String(params[i + 1].clone()),
                );
                i += 2;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    let output_format = agent_browser_hub::core::OutputFormat::from_str(format);

    match resolved {
        agent_browser_hub::registry::ResolvedCommand::Pipeline(script) => {
            let executor = agent_browser_hub::core::Executor::new().await?;
            let result = executor.execute(&script, param_map).await?;
            let formatted =
                agent_browser_hub::core::output::format_output(&result.result, &output_format)?;
            println!("{}", formatted);
            Ok(())
        }
        agent_browser_hub::registry::ResolvedCommand::WorkflowScript(target) => {
            let result =
                agent_browser_hub::workflow::runtime::execute_workflow_script(target, param_map)
                    .await?;
            let formatted =
                agent_browser_hub::core::output::format_output(&result, &output_format)?;
            println!("{}", formatted);
            Ok(())
        }
        agent_browser_hub::registry::ResolvedCommand::WorkflowNative(target) => {
            let result = agent_browser_hub::workflow::runtime::execute_native_dispatch(
                agent_browser_hub::workflow::runtime::NativeDispatchTarget::Workflow {
                    site: target.site,
                    name: target.name,
                    handler: target.handler,
                },
                param_map,
            )
            .await?;
            let formatted =
                agent_browser_hub::core::output::format_output(&result, &output_format)?;
            println!("{}", formatted);
            Ok(())
        }
        agent_browser_hub::registry::ResolvedCommand::Native(name) => {
            let result = agent_browser_hub::workflow::runtime::execute_native_dispatch(
                agent_browser_hub::workflow::runtime::NativeDispatchTarget::Registered {
                    handler: name,
                },
                param_map,
            )
            .await?;
            let formatted =
                agent_browser_hub::core::output::format_output(&result, &output_format)?;
            println!("{}", formatted);
            Ok(())
        }
    }
}

async fn cli_upgrade() -> anyhow::Result<()> {
    println!("Checking for updates...");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    let release: serde_json::Value = client
        .get(format!(
            "https://api.github.com/repos/{}/releases/latest",
            GITHUB_REPO
        ))
        .header("User-Agent", "agent-browser-hub")
        .send()
        .await?
        .json()
        .await?;

    let tag = release["tag_name"].as_str().unwrap_or("unknown");
    println!("Current: v{}", VERSION);
    println!("Latest:  {}", tag);

    let asset_name = if cfg!(target_arch = "x86_64") {
        "agent-browser-hub-linux-amd64"
    } else if cfg!(target_arch = "aarch64") {
        "agent-browser-hub-linux-arm64"
    } else {
        anyhow::bail!("Unsupported architecture");
    };

    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("No assets"))?;
    let download_url = assets
        .iter()
        .find(|a| a["name"].as_str() == Some(asset_name))
        .and_then(|a| a["browser_download_url"].as_str())
        .ok_or_else(|| anyhow::anyhow!("No binary for current architecture"))?;

    println!("Downloading {}...", asset_name);

    let download_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()?;

    let resp = download_client
        .get(download_url)
        .header("User-Agent", "agent-browser-hub")
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Download failed: {}", resp.status());
    }

    let binary_data = resp.bytes().await?;
    let temp_path = "/tmp/agent-browser-hub-new";
    std::fs::write(temp_path, &binary_data)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(temp_path, std::fs::Permissions::from_mode(0o755))?;
    }

    let output = tokio::process::Command::new(temp_path)
        .arg("version")
        .output()
        .await?;

    if !output.status.success() {
        std::fs::remove_file(temp_path)?;
        anyhow::bail!("New binary verification failed");
    }

    let current_exe = std::env::current_exe()?;
    let backup_path = format!("{}.bak", current_exe.display());

    std::fs::copy(&current_exe, &backup_path)?;
    println!("Backup created: {}", backup_path);

    std::fs::remove_file(&current_exe)?;
    if let Err(e) = std::fs::copy(temp_path, &current_exe) {
        let _ = std::fs::copy(&backup_path, &current_exe);
        anyhow::bail!("Failed to replace binary: {}", e);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&current_exe, std::fs::Permissions::from_mode(0o755))?;
    }

    std::fs::remove_file(temp_path)?;

    println!("Upgrade complete! Version: {}", tag);
    Ok(())
}
