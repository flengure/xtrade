// src/bot/cli/commands.rs

use crate::app_config::{AppConfig, LocalCliConfig, RemoteCliConfig};
use crate::app_state::AppState;
use crate::bot::rest::RestClient;
use crate::bot::state::{
    BotDeleteArgs, BotGetArgs, BotInsertArgs, BotListArgs, BotUpdateArgs, ListenerDeleteArgs,
    ListenerGetArgs, ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs, ListenersDeleteArgs,
    ServerStartupArgs,
};
use clap::{ArgGroup, Parser, Subcommand};
// use log::LevelFilter;
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};

/// Command-line interface for xTrade.
#[derive(Parser, Clone, Debug)]
#[command(
    name = "xtrade",
    about = "CLI tool for managing bots, listeners, and server"
)]
#[command(group(
    ArgGroup::new("backend")
        .args(&["state", "url"])
        .required(false)
        .multiple(false)
))]
pub struct Cli {
    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Use a local state file (future-proof for DB)
    #[arg(long, global = true)]
    pub state: Option<String>,

    /// Use a remote REST API endpoint
    #[arg(long, global = true)]
    pub url: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    // /// Determine the log level based on verbosity
    // pub fn log_level(&self) -> LevelFilter {
    //     match self.verbose {
    //         0 => LevelFilter::Warn,
    //         1 => LevelFilter::Info,
    //         2 => LevelFilter::Debug,
    //         _ => LevelFilter::Trace,
    //     }
    // }

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
            "offline" => run_offline_mode(self.clone(), app_config.local_cli, app_state).await,
            "online" => run_online_mode(self.clone(), app_config.remote_cli).await,
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
async fn run_offline_mode(
    cli: Cli,
    local_cli_config: LocalCliConfig,
    app_state: Arc<Mutex<AppState>>,
) -> Result<()> {
    if let Commands::Offline { offline_command } = cli.command {
        super::local_client::run(offline_command, local_cli_config, app_state)
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
async fn run_online_mode(cli: Cli, remote_cli_config: RemoteCliConfig) -> Result<()> {
    let rest_client = RestClient::new(&cli.url.unwrap_or(remote_cli_config.url));
    super::remote_client::run(cli.command, rest_client)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))
}
