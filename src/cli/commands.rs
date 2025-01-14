// cli/commands.rs
use crate::utils::validators::{validate_bind_address, validate_port, validate_url};
use clap::{ArgGroup, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "xtrade")]
#[command(about = "CLI tool for managing bots and listeners")]
#[command(about = "Manage xTrade bots and listeners")]
pub struct Cli {
    /// Online mode arguments (default if no explicit mode is provided)
    #[command(flatten)]
    pub online: OnlineArgs,

    /// Subcommands for different modes
    #[command(subcommand)]
    pub mode: Option<Mode>,
}

/// Modes for xTrade
#[derive(Subcommand, Debug)]
pub enum Mode {
    #[command(about = "Run the server mode")]
    Server(ServerArgs),

    #[command(about = "Run manage bots and listeners locally")]
    Offline(OfflineArgs),

    #[command(about = "Run online mode to interact with the server (default mode)")]
    Online(OnlineArgs),
}

/// Common arguments for online mode
#[derive(Debug, Parser)]
pub struct OnlineArgs {
    #[arg(long, value_parser = validate_url, help = "The URL of the xTrade server")]
    pub url: Option<String>, // Optional but validated for correctness

    /// CLI commands for online mode
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Common arguments for offline mode
#[derive(Debug, Parser)]
pub struct OfflineArgs {
    /// Specify the state file for offline mode
    #[arg(long)]
    pub state: Option<String>,

    /// CLI commands for offline mode
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Server arguments
#[derive(Debug, Parser)]
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

/// Commands for managing bots and listeners
#[derive(Subcommand, Debug)]
pub enum Commands {
    ClearAll {
        #[arg(long)]
        target: String,
    },

    /// Add a new bot
    #[clap(about = "Adds a new bot with the specified details.")]
    AddBot {
        #[arg(long)]
        bot_id: Option<String>,

        /// Name of the bot
        #[arg(long)]
        name: String,

        /// Exchange of the bot
        #[arg(long)]
        exchange: String,

        /// API key (optional)
        #[arg(long)]
        api_key: Option<String>,

        /// API secret (optional)
        #[arg(long)]
        api_secret: Option<String>,

        /// REST endpoint (optional)
        #[arg(long)]
        rest_endpoint: Option<String>,

        /// RPC endpoint (optional)
        #[arg(long)]
        rpc_endpoint: Option<String>,

        /// Webhook secret (optional)
        #[arg(long)]
        webhook_secret: Option<String>,

        /// Trading fee (optional)
        #[arg(long)]
        trading_fee: Option<f64>,

        /// Private key (optional)
        #[arg(long)]
        private_key: Option<String>,

        /// Contract address (optional)
        #[arg(long)]
        contract_address: Option<String>,
    },

    /// List all bots
    #[clap(about = "Lists all available bots.")]
    ListBots,

    /// Get a bot by its ID
    GetBot {
        /// The ID of the bot to retrieve
        #[arg(long)]
        bot_id: String,
    },

    /// Update an existing bot
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

    /// Delete a bot
    DeleteBot {
        #[arg(long)]
        bot_id: String,
    },

    /// Add a listener to a bot
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

    /// List a bots listeners with optional id and service filters
    ListListeners {
        #[arg(long)]
        bot_id: String,

        #[arg(long)]
        listener_id: Option<String>,

        #[arg(long)]
        service: Option<String>,
    },

    /// Get a listener from a bots listeners with optional id and service filters
    GetListener {
        #[arg(long)]
        bot_id: String,

        #[arg(long)]
        listener_id: Option<String>,

        #[arg(long)]
        service: Option<String>,
    },

    /// Update an existing listener
    UpdateListener {
        #[arg(long)]
        bot_id: Option<String>,

        #[arg(long)]
        listener_id: String,

        #[arg(long)]
        service: Option<String>,

        #[arg(long)]
        secret: Option<String>,

        #[arg(long)]
        msg: Option<String>,
    },

    /// Delete a listener
    DeleteListeners {
        bot_id: String,

        #[arg(long)]
        listener_id: Option<String>,

        #[arg(long)]
        service: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// Test parsing online mode with a URL and a command
    #[test]
    fn test_cli_parses_online_mode_with_url_and_command() {
        let args = vec![
            "xtrade",
            "--url",
            "http://example.com",
            "add-bot",
            "--name",
            "TestBot",
            "--exchange",
            "Binance",
        ];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.online.url.as_deref(), Some("http://example.com"));

        if let Some(Commands::AddBot { name, exchange, .. }) = cli.online.command {
            assert_eq!(name, "TestBot");
            assert_eq!(exchange, "Binance");
        } else {
            panic!("Command not parsed correctly as AddBot");
        }
    }

    /// Test parsing offline mode with a state file and a command
    #[test]
    fn test_cli_parses_offline_mode() {
        let args = vec![
            "xtrade",
            "offline",
            "--state",
            "offline_state.json",
            "list-bots",
        ];
        let cli = Cli::parse_from(args);

        if let Some(Mode::Offline(OfflineArgs { state, command })) = cli.mode {
            assert_eq!(state.as_deref(), Some("offline_state.json"));
            if let Some(Commands::ListBots) = command {
                assert!(true); // Successfully parsed ListBots
            } else {
                panic!("Command not parsed correctly as ListBots");
            }
        } else {
            panic!("Mode not parsed correctly as Offline");
        }
    }

    /// Test parsing server mode with required arguments
    #[test]
    fn test_cli_parses_server_mode() {
        let args = vec![
            "xtrade",
            "server",
            "--port",
            "9090",
            "--bind",
            "127.0.0.1",
            "--state",
            "server_state.json",
        ];
        let cli = Cli::parse_from(args);

        if let Some(Mode::Server(ServerArgs {
            port, bind, state, ..
        })) = cli.mode
        {
            assert_eq!(port, Some(9090));
            assert_eq!(bind.as_deref(), Some("127.0.0.1"));
            assert_eq!(state.as_deref(), Some("server_state.json"));
        } else {
            panic!("Mode not parsed correctly as Server");
        }
    }

    /// Test the default mode as online with no explicit mode provided
    #[test]
    fn test_cli_defaults_to_online_mode() {
        let args = vec!["xtrade", "list-bots"];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.online.url, None); // Default URL not specified
        if let Some(Commands::ListBots) = cli.online.command {
            assert!(true); // Successfully parsed ListBots in online mode
        } else {
            panic!("Default mode not parsed correctly as Online with ListBots");
        }
    }

    /// Test the URL validator
    #[test]
    fn test_validate_url_parser() {
        // Use the `validate_url` parser directly
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("invalid-url").is_err());
    }

    /// Test parsing the ClearAll command
    #[test]
    fn test_clear_all_command() {
        let args = vec![
            "xtrade",
            "offline",
            "--state",
            "offline_state.json",
            "clear-all",
            "--target",
            "bots",
        ];
        let cli = Cli::parse_from(args);

        if let Some(Mode::Offline(OfflineArgs {
            state,
            command: Some(Commands::ClearAll { target }),
        })) = cli.mode
        {
            // Validate the state file and target
            assert_eq!(state.as_deref(), Some("offline_state.json"));
            assert_eq!(target, "bots");
        } else {
            panic!("Command not parsed correctly as ClearAll");
        }
    }
}
