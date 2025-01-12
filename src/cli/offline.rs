// src/cli/offline.rs
use crate::app_state::AppState;
use crate::cli::Commands;
use crate::cli::OfflineArgs;
use crate::models::{Bot, Listener};
use config::{Config, File};
use serde_json::Value;

pub fn offline_mode(args: OfflineArgs) {
    // Load settings from config.toml
    let settings = Config::builder()
        .add_source(File::with_name("cli").required(false)) // Load cli.toml
        .build()
        .unwrap();

    // Fallback to default in config.toml if --state is not provided
    let default_state = settings
        .get_string("cli.state")
        .unwrap_or_else(|_| "state.json".to_string());
    let state_file = args.state.unwrap_or(default_state);

    log::info!("Using state file: {}", state_file);

    // Load or initialize AppState
    let mut app_state =
        AppState::load_state_from_file(&state_file).unwrap_or_else(|_| AppState::default());

    if let Some(command) = args.command {
        handle_offline_mode(command, &mut app_state, &state_file);
    } else {
        println!("No command provided for offline mode.");
    }
}

/// Handle CLI commands in offline mode
pub fn handle_offline_mode(command: Commands, app_state: &mut AppState, state_file: &str) {
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
            let bot = Bot {
                bot_id: uuid::Uuid::new_v4().to_string(),
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
                listeners: Vec::new(),
            };
            let bot_id = app_state.add_bot(bot);
            app_state.save_state_to_file(state_file);
            println!("Bot added successfully with ID: {}", bot_id);
        }

        Commands::ListBots => {
            for bot in app_state.list_bots() {
                println!("{:?}", bot);
            }
        }

        Commands::GetBot { bot_id } => match app_state.get_bot(&bot_id) {
            Some(bot) => println!("{:?}", bot),
            None => println!("Error: Bot with ID {} not found.", bot_id),
        },

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
            if let Some(mut bot) = app_state.get_bot(&bot_id).cloned() {
                if let Some(name) = name {
                    bot.name = name;
                }
                if let Some(exchange) = exchange {
                    bot.exchange = exchange;
                }
                if let Some(api_key) = api_key {
                    bot.api_key = Some(api_key);
                }
                if let Some(api_secret) = api_secret {
                    bot.api_secret = Some(api_secret);
                }
                if let Some(rest_endpoint) = rest_endpoint {
                    bot.rest_endpoint = Some(rest_endpoint);
                }
                if let Some(rpc_endpoint) = rpc_endpoint {
                    bot.rpc_endpoint = Some(rpc_endpoint);
                }
                if let Some(webhook_secret) = webhook_secret {
                    bot.webhook_secret = Some(webhook_secret);
                }
                if let Some(trading_fee) = trading_fee {
                    bot.trading_fee = Some(trading_fee);
                }
                if let Some(private_key) = private_key {
                    bot.private_key = Some(private_key);
                }
                if let Some(contract_address) = contract_address {
                    bot.contract_address = Some(contract_address);
                }

                if app_state.update_bot(&bot_id, bot).is_ok() {
                    app_state.save_state_to_file(state_file);
                    println!("Bot updated successfully.");
                } else {
                    println!("Error: Failed to update bot.");
                }
            } else {
                println!("Error: Bot with ID {} not found.", bot_id);
            }
        }

        Commands::DeleteBot { bot_id } => {
            if app_state.delete_bot(&bot_id).is_ok() {
                app_state.save_state_to_file(state_file);
                println!("Bot deleted successfully.");
            } else {
                println!("Error: Bot with ID {} not found.", bot_id);
            }
        }

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => {
            if let Some(bot_id) = app_state.get_bot(&bot_id).map(|bot| bot.bot_id.clone()) {
                let listener = Listener {
                    listener_id: uuid::Uuid::new_v4().to_string(),
                    service,
                    secret: secret.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                    msg: msg
                        .and_then(|m| serde_json::from_str::<Value>(&m).ok())
                        .unwrap_or(Value::Null),
                };

                if app_state.add_listener(&bot_id, listener).is_ok() {
                    app_state.save_state_to_file(state_file);
                    println!("Listener added successfully.");
                } else {
                    println!("Error: Failed to add listener.");
                }
            } else {
                println!("Error: Bot with ID {} not found.", bot_id);
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

        Commands::GetListener { listener_id } => match app_state.get_listener(&listener_id) {
            Some(listener) => println!("{:?}", listener),
            None => println!("Error: Listener with ID {} not found.", listener_id),
        },

        Commands::UpdateListener {
            listener_id,
            service,
            secret,
            msg,
        } => {
            if let Some(listener) = app_state.get_listener(&listener_id).cloned() {
                let updated_listener = Listener {
                    service: service.unwrap_or(listener.service),
                    secret: secret.unwrap_or(listener.secret),
                    msg: msg
                        .and_then(|m| serde_json::from_str::<Value>(&m).ok())
                        .unwrap_or(listener.msg),
                    ..listener
                };

                if app_state
                    .update_listener(&listener_id, updated_listener)
                    .is_ok()
                {
                    app_state.save_state_to_file(state_file);
                    println!("Listener updated successfully.");
                } else {
                    println!("Error: Failed to update listener.");
                }
            } else {
                println!("Error: Listener with ID {} not found.", listener_id);
            }
        }

        Commands::DeleteListener { listener_id } => {
            if app_state.delete_listener(&listener_id).is_ok() {
                app_state.save_state_to_file(state_file);
                println!("Listener deleted successfully.");
            } else {
                println!("Error: Listener with ID {} not found.", listener_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::AppState;
    use crate::cli::Commands;
    use crate::models::{Bot, Listener};
    use serde_json::json;
    use std::collections::HashMap;

    /// Mock an empty `AppState`
    fn mock_app_state() -> AppState {
        AppState {
            bots: HashMap::new(),
        }
    }

    #[test]
    fn test_offline_mode_with_default_state() {
        let args = OfflineArgs {
            state: None, // No state file provided
            command: Some(Commands::ListBots),
        };

        let app_state = mock_app_state();

        // Mock default state from config.toml
        let default_state = "state.json";

        // Redirect log output
        let state_file = args.state.clone().unwrap_or(default_state.to_string());

        assert_eq!(state_file, "state.json");
        assert_eq!(app_state.list_bots().len(), 0); // Initial state
    }

    #[test]
    fn test_offline_mode_with_custom_state() {
        let args = OfflineArgs {
            state: Some("custom_state.json".to_string()), // Custom state file provided
            command: Some(Commands::ListBots),
        };

        let state_file = args.state.clone().unwrap_or("state.json".to_string());
        assert_eq!(state_file, "custom_state.json");
    }

    #[test]
    fn test_handle_offline_mode_add_bot() {
        let mut app_state = mock_app_state();
        let command = Commands::AddBot {
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.1),
            private_key: None,
            contract_address: None,
        };

        handle_offline_mode(command, &mut app_state, "state.json");
        let bots = app_state.list_bots();
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].name, "TestBot");
    }

    #[test]
    fn test_handle_offline_mode_list_bots() {
        let mut app_state = mock_app_state();
        app_state.add_bot(Bot {
            bot_id: "bot123".to_string(),
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.1),
            private_key: None,
            contract_address: None,
            listeners: Vec::new(),
        });

        handle_offline_mode(Commands::ListBots, &mut app_state, "state.json");
        let bots = app_state.list_bots();
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].name, "TestBot");
    }

    #[test]
    fn test_handle_offline_mode_update_bot() {
        let mut app_state = mock_app_state();
        let bot_id = "bot123".to_string();
        app_state.add_bot(Bot {
            bot_id: bot_id.clone(),
            name: "OldName".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.1),
            private_key: None,
            contract_address: None,
            listeners: Vec::new(),
        });

        handle_offline_mode(
            Commands::UpdateBot {
                bot_id: bot_id.clone(),
                name: Some("NewName".to_string()),
                exchange: None,
                api_key: None,
                api_secret: None,
                rest_endpoint: None,
                rpc_endpoint: None,
                webhook_secret: None,
                trading_fee: None,
                private_key: None,
                contract_address: None,
            },
            &mut app_state,
            "state.json",
        );

        let bots = app_state.list_bots();
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].name, "NewName");
    }

    #[test]
    fn test_handle_offline_mode_delete_bot() {
        let mut app_state = mock_app_state();
        let bot_id = "bot123".to_string();
        app_state.add_bot(Bot {
            bot_id: bot_id.clone(),
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.1),
            private_key: None,
            contract_address: None,
            listeners: Vec::new(),
        });

        handle_offline_mode(Commands::DeleteBot { bot_id }, &mut app_state, "state.json");
        assert_eq!(app_state.list_bots().len(), 0);
    }

    #[test]
    fn test_handle_offline_mode_add_listener() {
        let mut app_state = mock_app_state();
        let bot_id = "bot123".to_string();
        app_state.add_bot(Bot {
            bot_id: bot_id.clone(),
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.1),
            private_key: None,
            contract_address: None,
            listeners: Vec::new(),
        });

        handle_offline_mode(
            Commands::AddListener {
                bot_id: bot_id.clone(),
                service: "TestService".to_string(),
                secret: None,
                msg: Some(json!({"key": "value"}).to_string()),
            },
            &mut app_state,
            "state.json",
        );

        let bots = app_state.list_bots();
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].listeners.len(), 1);
        assert_eq!(bots[0].listeners[0].service, "TestService");
    }

    #[test]
    fn test_handle_offline_mode_delete_listener() {
        let mut app_state = mock_app_state();
        let bot_id = "bot123".to_string();
        let listener_id = "listener123".to_string();
        app_state.add_bot(Bot {
            bot_id: bot_id.clone(),
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.1),
            private_key: None,
            contract_address: None,
            listeners: vec![Listener {
                listener_id: listener_id.clone(),
                service: "TestService".to_string(),
                secret: "Secret123".to_string(),
                msg: Value::Null,
            }],
        });

        handle_offline_mode(
            Commands::DeleteListener {
                listener_id: listener_id.clone(),
            },
            &mut app_state,
            "state.json",
        );

        let bots = app_state.list_bots();
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].listeners.len(), 0);
    }
}
