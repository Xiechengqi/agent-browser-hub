use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agent-browser-hub")]
#[command(about = "Browser automation scripts hub")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start web server
    Serve {
        #[arg(long, default_value = "3133")]
        port: u16,
    },
    /// List all available scripts
    List,
    /// Show current version
    Version,
    /// Upgrade to latest release from GitHub
    Upgrade,
    /// Run a script
    Run {
        /// Script path in format: site/command
        script: String,
        /// Output format: json, yaml, table, csv, md
        #[arg(long, default_value = "json")]
        format: String,
        /// Parameters in format: --key value
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        params: Vec<String>,
    },
}
