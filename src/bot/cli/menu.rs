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
    /// Use a local state file (future-proof for DB)
    #[arg(long)]
    pub state: Option<String>,

    /// Use a remote REST API endpoint
    #[arg(long)]
    pub url: Option<String>,

    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Subcommands for managing bots, listeners, and server
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    /// Determine the mode (offline, online, or server).
    pub fn mode(&self) -> &str {
        match (&self.state, &self.url, &self.command) {
            (_, _, Commands::Server { .. }) => "server",
            (Some(_), None, _) => "offline",
            (None, Some(_), _) => "online",
            (None, None, _) => "offline",
            _ => unreachable!("Invalid combination of 'state' and 'url'."),
        }
    }
    pub fn log_level(&self) -> LevelFilter {
        match &self.command {
            Commands::Server { .. } => match self.verbose {
                0 => LevelFilter::Info,  // Default: Show info messages for server
                1 => LevelFilter::Debug, // -v: Show debug and info messages
                _ => LevelFilter::Trace, // -vv or more: Show everything
            },
            _ => match self.verbose {
                0 => LevelFilter::Warn,  // Default: Only warnings and errors
                1 => LevelFilter::Info,  // -v: Show info, warnings, and errors
                2 => LevelFilter::Debug, // -vv: Show debug, info, warnings, and errors
                _ => LevelFilter::Trace, // -vvv or more: Show everything
            },
        }
    }
}

/// Commands for managing bots, listeners, and server
#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    /// Clears all bots or listeners
    ClearAll {
        /// Target to clear: "bots" or "listeners"
        #[arg(long, value_parser = ["bots", "listeners"], required = true)]
        target: String,
    },

    /// Runs the xTrade server
    Server {
        #[arg(long)]
        port: Option<u16>,
        #[arg(long)]
        bind: Option<String>,
        #[arg(long)]
        state: Option<String>,
        /// Enable the Web UI (default)
        #[arg(long, conflicts_with = "no_web")]
        web: bool,
        /// Disable the Web UI
        #[arg(long, conflicts_with = "web")]
        no_web: bool,
        #[arg(long)]
        web_port: Option<u16>,
        #[arg(long)]
        web_bind: Option<String>,
        #[arg(long)]
        web_path: Option<String>,
    },

    // Bot Commands
    AddBot {
        #[arg(long)]
        bot_id: Option<String>,
        #[arg(long)]
        name: String,
        #[arg(long)]
        exchange: String,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        api_secret: Option<String>,
        #[arg(long)]
        rest_endpoint: Option<String>,
        #[arg(long)]
        rpc_endpoint: Option<String>,
        #[arg(long)]
        webhook_secret: Option<String>,
        #[arg(long)]
        trading_fee: Option<f64>,
        #[arg(long)]
        private_key: Option<String>,
        #[arg(long)]
        contract_address: Option<String>,
    },

    ListBots {
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        bot_id: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        exchange: Option<String>,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        rest_endpoint: Option<String>,
        #[arg(long)]
        rpc_endpoint: Option<String>,
        #[arg(long)]
        trading_fee: Option<f64>,
        #[arg(long)]
        private_key: Option<String>,
        #[arg(long)]
        contract_address: Option<String>,
    },

    GetBot {
        #[arg(long)]
        bot_id: String,
    },

    UpdateBot {
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        exchange: Option<String>,
        #[arg(long)]
        api_key: Option<String>,
        #[arg(long)]
        api_secret: Option<String>,
        #[arg(long)]
        rest_endpoint: Option<String>,
        #[arg(long)]
        rpc_endpoint: Option<String>,
        #[arg(long)]
        webhook_secret: Option<String>,
        #[arg(long)]
        trading_fee: Option<f64>,
        #[arg(long)]
        private_key: Option<String>,
        #[arg(long)]
        contract_address: Option<String>,
    },

    DeleteBot {
        #[arg(long)]
        bot_id: String,
    },

    // Listener Commands
    AddListener {
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        service: String,
        #[arg(long)]
        secret: Option<String>,
        #[arg(long)]
        msg: Option<String>,
    },

    ListListeners {
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        listener_id: Option<String>,
        #[arg(long)]
        service: Option<String>,
    },

    GetListener {
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        listener_id: String,
    },

    UpdateListener {
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        listener_id: String,
        #[arg(long)]
        service: Option<String>,
        #[arg(long)]
        secret: Option<String>,
        #[arg(long)]
        msg: Option<String>,
    },

    DeleteListener {
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        listener_id: String,
    },

    DeleteListeners {
        #[arg(long)]
        bot_id: String,
        #[arg(long)]
        listener_id: Option<String>,
        #[arg(long)]
        service: Option<String>,
    },
}
