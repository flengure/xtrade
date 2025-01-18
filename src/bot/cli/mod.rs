// src/bot/cli/mod.rs
pub mod cmd;
pub mod menu;
pub mod offline;
pub mod online;
pub use cmd::run;
pub use menu::Cli;
use menu::Commands;

use crate::utils::validators::{validate_bind_address, validate_port, validate_url};
use clap::{ArgGroup, Parser, Subcommand};

/// Modes for xTrade
#[derive(Subcommand, Clone, Debug)]
pub enum Mode {
    #[command(about = "Run the server mode")]
    Server(ServerArgs),

    #[command(about = "Manage bots and listeners locally (default mode)")]
    Offline(OfflineArgs),

    #[command(about = "Run online mode to interact with the server")]
    Online(OnlineArgs),
}

/// Common arguments for online mode
#[derive(Debug, Clone, Parser)]
pub struct OnlineArgs {
    #[arg(long, value_parser = validate_url, help = "The URL of the xTrade server")]
    pub url: Option<String>, // Optional but validated for correctness

    /// CLI commands for online mode
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Common arguments for offline mode
#[derive(Debug, Clone, Parser)]
pub struct OfflineArgs {
    /// Specify the state file for offline mode
    #[arg(long)]
    pub state: Option<String>,

    /// CLI commands for offline mode
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Server arguments
#[derive(Parser, Clone, Debug)]
#[command(group(
    ArgGroup::new("web_toggle")
        .args(&["web", "no_web"])
        .multiple(false)
))]
pub struct ServerArgs {
    /// Specify the port for the server
    #[arg(long, value_parser = validate_port)]
    pub port: Option<u16>, // Default value and validated for range

    /// Specify the bind address for the server
    #[arg(long, value_parser = validate_bind_address)]
    pub bind: Option<String>, // Default value and validated for correctness

    /// Specify the state file for the server
    #[arg(long, default_value = "state.json")]
    pub state: Option<String>, // Default value

    /// Enable the Web UI (enabled by default)
    #[arg(long, group = "web_toggle")]
    pub web: bool,

    /// Disable the Web UI
    #[arg(long, group = "web_toggle")]
    pub no_web: bool,

    /// Specify the port for the web client
    #[arg(long, value_parser = validate_port)]
    pub web_port: Option<u16>, // Default value and validated for range

    /// Specify the bind address for the web client
    #[arg(long, value_parser = validate_bind_address)]
    pub web_bind: Option<String>, // Default value and validated for correctness

    /// Specify the root for the web client
    #[arg(long)]
    pub web_root: Option<String>, // Default value
}
