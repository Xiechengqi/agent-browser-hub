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
}
