use agent_browser_hub::cli::{Cli, Commands};
use agent_browser_hub::server;
use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "Xiechengqi/agent-browser-cli";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Serve { port }) => {
            server::start(port).await?;
        }
        Some(Commands::List) => {
            println!("Available scripts:");
            println!("  google/search     - Search Google");
            println!("  hackernews/top    - Hacker News top stories");
        }
        Some(Commands::Version) => {
            println!("agent-browser-hub v{}", VERSION);
        }
        Some(Commands::Upgrade) => {
            cli_upgrade().await?;
        }
        None => {
            println!("agent-browser-hub v{}", VERSION);
            println!("Use --help for usage");
        }
    }

    Ok(())
}

async fn cli_upgrade() -> anyhow::Result<()> {
    println!("Checking for updates...");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    // 1. Fetch latest release
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

    // 2. Find asset
    let asset_name = if cfg!(target_arch = "x86_64") {
        "agent-browser-hub-linux-amd64"
    } else if cfg!(target_arch = "aarch64") {
        "agent-browser-hub-linux-arm64"
    } else {
        anyhow::bail!("Unsupported architecture");
    };

    let assets = release["assets"].as_array().ok_or_else(|| anyhow::anyhow!("No assets"))?;
    let download_url = assets
        .iter()
        .find(|a| a["name"].as_str() == Some(asset_name))
        .and_then(|a| a["browser_download_url"].as_str())
        .ok_or_else(|| anyhow::anyhow!("No binary for current architecture"))?;

    println!("Downloading {}...", asset_name);

    // 3. Download
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

    // 4. Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(temp_path, std::fs::Permissions::from_mode(0o755))?;
    }

    // 5. Verify
    let output = tokio::process::Command::new(temp_path)
        .arg("version")
        .output()
        .await?;

    if !output.status.success() {
        std::fs::remove_file(temp_path)?;
        anyhow::bail!("New binary verification failed");
    }

    // 6. Replace
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
