
use crate::app_state::AppState;
use crate::models::{Bot, Listener};
use clap::{Parser, Subcommand};
use log::info;
use std::sync::Mutex;
use uuid::Uuid;

/// CLI tool for managing bots and listeners
#[derive(Parser, Debug)]
#[command(name = "xtrade")]
#[command(about = "Manage xTrade bots and listeners")]
pub struct Cli {
    /// Run the server
    #[arg(long)]
    pub server: bool,

    /// Run in offline mode
    #[arg(long)]
    pub offline: bool,

    /// Specify the server port
    #[arg(long)]
    pub port: Option<u16>,

    /// Specify the state file
    #[arg(long)]
    pub state: Option<String>,

    /// Specify the server URL (for online mode)
    #[arg(long)]
    pub url: Option<String>,

    /// CLI Commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Commands for the CLI
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new bot
    AddBot {
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
    ListBots,

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
    /// List all listeners for a bot
    ListListeners { bot_id: String },
}

/// Handle CLI commands in offline mode
pub fn handle_offline_mode(cli: Cli, app_state: &mut AppState, state_file: &str) {
    if let Some(command) = cli.command {
        match command {
            Commands::AddBot {
                name,
                exchange,
                api_key,
                api_secret,
                rest_endpoint,
                rpc_endpoint,
                webhook_secret,
                trading_fee,
                private_key,
                contract_address,
            } => {
                let bot_id = app_state.add_bot(
                    name,
                    exchange,
                    api_key,
                    api_secret,
                    rest_endpoint,
                    rpc_endpoint,
                    webhook_secret,
                    trading_fee.unwrap_or(0.1),
                    private_key,
                    contract_address,
                );
                app_state.save_state_to_file(state_file);
                println!("Bot added successfully with ID: {}", bot_id);
            }
            Commands::ListBots => {
                for bot in app_state.list_bots() {
                    println!("{:?}", bot);
                }
            }
            Commands::AddListener {
                bot_id,
                service,
                secret,
                msg,
            } => {
                match app_state.add_listener(&bot_id, service, secret, msg.map(serde_json::from_str).unwrap_or_else(|_| serde_json::Value::Null)) {
                    Ok(listener_id) => {
                        app_state.save_state_to_file(state_file);
                        println!("Listener added successfully with ID: {}", listener_id);
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }
            Commands::ListListeners { bot_id } => match app_state.list_listeners(&bot_id) {
                Ok(listeners) => {
                    for listener in listeners {
                        println!("{:?}", listener);
                    }
                }
                Err(err) => println!("Error: {}", err),
            },
        }
    }
}

/// Handle CLI commands in online mode
pub fn handle_online_mode(cli: Cli, url: &str) {
    if let Some(command) = cli.command {
        match command {
            Commands::AddBot { .. } => {
                println!("AddBot command is not yet implemented for online mode.");
            }
            Commands::ListBots => {
                println!("Fetching bots from {}", url);
            }
            Commands::AddListener { .. } => {
                println!("AddListener command is not yet implemented for online mode.");
            }
            Commands::ListListeners { .. } => {
                println!("ListListeners command is not yet implemented for online mode.");
            }
        }
    }
}

/// Start the server in server mode
pub async fn start_server(app_state: AppState, port: u16) -> std::io::Result<()> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Mutex::new(app_state.clone()))
            .configure(crate::api::init_routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
