// src/bot/cli/commands.rs

use crate::app_config::AppConfig;
use crate::app_state::AppState;
use crate::bot::rest::RestClient;
use crate::bot::state::{
    BotDeleteArgs, BotGetArgs, BotInsertArgs, BotListArgs, BotUpdateArgs, ListenerDeleteArgs,
    ListenerGetArgs, ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs, ListenersDeleteArgs,
    ServerStartupArgs,
};
use clap::{Parser, Subcommand};
// use log::LevelFilter;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Command-line interface for xTrade.
#[derive(Parser, Clone, Debug)]
#[command(
    name = "xtrade",
    about = "CLI tool for managing bots, listeners, and server"
)]
pub struct Cli {
    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Use a remote REST API endpoint
    #[arg(long)]
    pub url: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Determine the CLI mode (offline or server)
    pub fn mode(&self) -> &str {
        match &self.command {
            Commands::Server { .. } => "server",
            Commands::Offline { .. } => "offline",
            _ => "online", // Default to "online" for all other commands
        }
    }

    /// Handles the CLI commands and modes
    pub async fn run(&self, app_config: AppConfig, app_state: Arc<Mutex<AppState>>) -> Result<()> {
        match self.mode() {
            "server" => run_server_mode(self.clone(), app_config, app_state).await,
            "offline" => run_offline_mode(self.clone()).await,
            "online" => run_online_mode(self.clone(), app_config).await,
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown or unsupported mode.",
            )),
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Offline mode commands
    Offline {
        /// Use a local state file (future-proof for DB)
        #[arg(long)]
        state_file: Option<PathBuf>,

        #[command(subcommand)]
        offline_command: OfflineCmds,
    },
    // Server
    Server(ServerStartupArgs),
    // Online mode commands (mirrors offline commands but acts through REST)
    AddBot(BotInsertArgs),
    ListBots(BotListArgs),
    GetBot(BotGetArgs),
    UpdateBot(BotUpdateArgs),
    DeleteBot(BotDeleteArgs),
    AddListener(ListenerInsertArgs),
    ListListeners(ListenerListArgs),
    GetListener(ListenerGetArgs),
    UpdateListener(ListenerUpdateArgs),
    DeleteListener(ListenerDeleteArgs),
    DeleteListeners(ListenersDeleteArgs),
}

#[derive(Subcommand, Clone, Debug)]
pub enum OfflineCmds {
    /// Clears all bots or listeners
    ClearAll {
        /// Target to clear: "bots" or "listeners"
        #[arg(long, value_parser = ["bots", "listeners"], required = true)]
        target: String,
    },
    AddBot(BotInsertArgs),
    ListBots(BotListArgs),
    GetBot(BotGetArgs),
    UpdateBot(BotUpdateArgs),
    DeleteBot(BotDeleteArgs),
    AddListener(ListenerInsertArgs),
    ListListeners(ListenerListArgs),
    GetListener(ListenerGetArgs),
    UpdateListener(ListenerUpdateArgs),
    DeleteListener(ListenerDeleteArgs),
    DeleteListeners(ListenersDeleteArgs),
}

/// Handle server mode
async fn run_server_mode(
    cli: Cli,
    app_config: AppConfig,
    app_state: Arc<Mutex<AppState>>,
) -> Result<()> {
    if let Commands::Server(server_args) = cli.command {
        super::server::run(server_args, app_config, app_state).await?;
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid command for server mode.",
        ))
    }
}

/// Handle offline mode
async fn run_offline_mode(cli: Cli) -> Result<()> {
    if let Commands::Offline {
        state_file,
        offline_command,
    } = cli.command
    {
        // Convert state_file from Option<PathBuf> to Option<&Path>
        let state_file = state_file.as_deref();

        super::local_client::run(state_file, offline_command)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid command for offline mode.",
        ))
    }
}

/// Handle online mode
async fn run_online_mode(cli: Cli, app_config: AppConfig) -> Result<()> {
    let rest_client = RestClient::new(&cli.url.unwrap_or(app_config.remote_cli.url));
    super::remote_client::run(cli.command, rest_client)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))
}
