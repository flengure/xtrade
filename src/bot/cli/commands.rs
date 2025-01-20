// src/bot/cli/commands.rs

use crate::bot::state::{
    BotDeleteArgs, BotGetArgs, BotInsertArgs, BotListArgs, BotUpdateArgs, ListenerDeleteArgs,
    ListenerGetArgs, ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs, ListenersDeleteArgs,
    ServerStartupArgs,
};
use clap::{ArgGroup, Parser, Subcommand};
use log::LevelFilter;

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
    #[arg(long)]
    pub state: Option<String>,

    /// Use a remote REST API endpoint
    #[arg(long)]
    pub url: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Determine the log level based on verbosity
    pub fn log_level(&self) -> LevelFilter {
        match self.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }

    /// Determine the CLI mode (offline, online, or server)
    pub fn mode(&self) -> &str {
        match (&self.state, &self.url, &self.command) {
            (_, _, Commands::Server { .. }) => "server",
            (Some(_), None, _) => "offline",
            (None, Some(_), _) => "online",
            (None, None, _) => "offline",
            _ => unreachable!("Invalid combination of 'state' and 'url'."),
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
