// src/cli/offline.rs

use crate::app_state::AppState;
use crate::cli::Commands;
use crate::cli::OfflineArgs;
use crate::models::{BotInsert, BotUpdate, ListenerInsert, ListenerList, ListenerUpdate};
use crate::utils::config::AppConfig;

pub fn offline_mode(args: OfflineArgs) {
    // Load application configuration or fallback to defaults
    let config = AppConfig::load(None).unwrap_or_else(|err| {
        log::warn!(
            "Failed to load configuration file: {}. Using defaults.",
            err
        );
        AppConfig::default()
    });

    // Use the configured offline state or override with CLI argument
    let state_file = args.state.unwrap_or_else(|| config.offline.state.clone());

    log::info!("Using state file: {}", state_file);

    // Load or initialize the application state
    let mut app_state = AppState::load(Some(&state_file)).unwrap_or_else(|_| AppState::default());

    if let Some(command) = args.command {
        handle_offline_mode(command, &mut app_state);
    } else {
        println!("No command provided for offline mode.");
    }
}

/// Handle CLI commands in offline mode
pub fn handle_offline_mode(command: Commands, app_state: &mut AppState) {
    match command {
        Commands::ClearAll { target } => match target.as_str() {
            "bots" => {
                app_state.clear_bots();
                println!("All bots cleared.");
            }
            "listeners" => {
                app_state.clear_listeners();
                println!("All listeners cleared.");
            }
            _ => eprintln!("Invalid target. Use 'bots' or 'listeners'."),
        },

        Commands::AddBot {
            bot_id,
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
            match app_state.create_bot(BotInsert {
                bot_id: Some(bot_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string())),
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
            }) {
                Ok(bot) => println!("Success, added Bot with ID: {}", bot.bot_id),
                Err(err) => eprintln!("Error: {}", err),
            }
        }

        Commands::UpdateBot {
            bot_id,
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
            match app_state.update_bot(BotUpdate {
                bot_id,
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
            }) {
                Ok(bot) => println!("Success, updated Bot with ID: {}", bot.bot_id),
                Err(err) => eprintln!("Error: {}", err),
            }
        }

        Commands::ListBots => {
            let bots = app_state.list_bots();
            if bots.is_empty() {
                println!("No bots available.");
            } else {
                println!("Available Bots:");
                for bot in bots {
                    println!(
                        "ID: {}, Name: {}, Exchange: {}",
                        bot.bot_id, bot.name, bot.exchange
                    );
                }
            }
        }

        Commands::GetBot { bot_id } => match app_state.get_bot(&bot_id) {
            Some(bot) => println!("{:?}", bot),
            None => eprintln!("Error: Bot ID: {} not found.", bot_id),
        },

        Commands::DeleteBot { bot_id } => match app_state.delete_bot(&bot_id) {
            Ok(_) => println!("Bot with ID '{}' deleted successfully.", bot_id),
            Err(err) => eprintln!("Error: {}", err), // Standardize error output for CLI
        },

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => match app_state.add_listener(ListenerInsert {
            bot_id,
            service,
            secret,
            msg,
        }) {
            Ok(listener) => println!(
                "Success, Listener ID: {} on Bot ID: {}",
                listener.listener_id, listener.bot_id
            ),
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::ListListeners {
            bot_id,
            listener_id,
            service,
        } => {
            match app_state.list_listeners(ListenerList {
                bot_id,
                listener_id,
                service,
            }) {
                Ok(listeners) => {
                    if listeners.is_empty() {
                        println!("No listeners found.");
                    } else {
                        println!("Listeners:");
                        for listener in listeners {
                            println!(
                                "Listener ID: {}, Service: {}, Bot ID: {}",
                                listener.listener_id,
                                listener.service.unwrap_or_default(),
                                listener.bot_id
                            );
                        }
                    }
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }

        Commands::GetListener {
            bot_id,
            listener_id,
            service,
        } => match app_state.get_listener(ListenerList {
            bot_id,
            listener_id,
            service,
        }) {
            Ok(listener) => println!("{:?}", listener),
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::UpdateListener {
            bot_id,
            listener_id,
            service,
            secret,
            msg,
        } => match app_state.update_listener(ListenerUpdate {
            bot_id,
            listener_id,
            service,
            secret,
            msg,
        }) {
            Ok(listener) => println!("{:?}", listener),
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::DeleteListeners {
            bot_id,
            listener_id,
            service,
        } => match app_state.delete_listeners(ListenerList {
            bot_id,
            listener_id,
            service,
        }) {
            Ok(_) => println!("Listener deleted successfully."),
            Err(err) => println!("Error: {}", err),
        },
    }
}
