// src/cli/online.rs
use crate::cli::{Commands, OnlineArgs};
use config::{Config, File};
use log::info;
use reqwest::blocking::Client;
use serde_json::Value;

/// Run the application in online mode
pub fn online_mode(args: OnlineArgs) {
    // Load settings from cli.toml
    let settings = Config::builder()
        .add_source(File::with_name("cli").required(false)) // Load cli.toml
        .build()
        .unwrap();

    // Use the --url argument or fallback to the default in config.toml
    let default_url = settings
        .get_string("cli.url")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let server_url = args.url.unwrap_or(default_url);

    info!("Running in online mode with server URL: {}", server_url);

    if let Some(command) = args.command {
        handle_online_mode(&server_url, command);
    } else {
        println!("No command provided for online mode.");
    }
}

/// Handle CLI commands in online mode
pub fn handle_online_mode(url: &str, command: Commands) {
    let client = Client::new();

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
            let bot_data = serde_json::json!({
                "name": name,
                "exchange": exchange,
                "api_key": api_key,
                "api_secret": api_secret,
                "rest_endpoint": rest_endpoint,
                "rpc_endpoint": rpc_endpoint,
                "webhook_secret": webhook_secret,
                "trading_fee": trading_fee.unwrap_or(0.1),
                "private_key": private_key,
                "contract_address": contract_address,
            });

            match client.post(&format!("{}/bots", url)).json(&bot_data).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Bot added successfully.");
                    } else {
                        println!("Failed to add bot: {}", response.text().unwrap_or_default());
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::ListBots => match client.get(&format!("{}/bots", url)).send() {
            Ok(response) => {
                if response.status().is_success() {
                    let bots: Value = response.json().unwrap_or_else(
                        |_| serde_json::json!({ "error": "Failed to parse response." }),
                    );
                    println!("Bots: {}", bots);
                } else {
                    println!(
                        "Failed to fetch bots: {}",
                        response.text().unwrap_or_default()
                    );
                }
            }
            Err(err) => println!("Error: {}", err),
        },

        Commands::GetBot { bot_id } => {
            match client.get(&format!("{}/bots/{}", url, bot_id)).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        let bot: Value = response.json().unwrap_or_else(
                            |_| serde_json::json!({ "error": "Failed to parse response." }),
                        );
                        println!("Bot: {}", bot);
                    } else {
                        println!(
                            "Failed to fetch bot: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
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
            let bot_data = serde_json::json!({
                "name": name,
                "exchange": exchange,
                "api_key": api_key,
                "api_secret": api_secret,
                "rest_endpoint": rest_endpoint,
                "rpc_endpoint": rpc_endpoint,
                "webhook_secret": webhook_secret,
                "trading_fee": trading_fee,
                "private_key": private_key,
                "contract_address": contract_address,
            });

            match client
                .put(&format!("{}/bots/{}", url, bot_id))
                .json(&bot_data)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Bot updated successfully.");
                    } else {
                        println!(
                            "Failed to update bot: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::DeleteBot { bot_id } => {
            match client.delete(&format!("{}/bots/{}", url, bot_id)).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Bot deleted successfully.");
                    } else {
                        println!(
                            "Failed to delete bot: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => {
            let listener_data = serde_json::json!({
                "bot_id": bot_id,
                "service": service,
                "secret": secret,
                "msg": msg,
            });

            match client
                .post(&format!("{}/listeners", url))
                .json(&listener_data)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Listener added successfully.");
                    } else {
                        println!(
                            "Failed to add listener: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::ListListeners { bot_id } => {
            match client
                .get(&format!("{}/bots/{}/listeners", url, bot_id))
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let listeners: Value = response.json().unwrap_or_else(
                            |_| serde_json::json!({ "error": "Failed to parse response." }),
                        );
                        println!("Listeners: {}", listeners);
                    } else {
                        println!(
                            "Failed to fetch listeners: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::GetListener { listener_id } => {
            match client
                .get(&format!("{}/listeners/{}", url, listener_id))
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let listener: Value = response.json().unwrap_or_else(
                            |_| serde_json::json!({ "error": "Failed to parse response." }),
                        );
                        println!("Listener: {}", listener);
                    } else {
                        println!(
                            "Failed to fetch listener: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::UpdateListener {
            listener_id,
            service,
            secret,
            msg,
        } => {
            let listener_data = serde_json::json!({
                "service": service,
                "secret": secret,
                "msg": msg,
            });

            match client
                .put(&format!("{}/listeners/{}", url, listener_id))
                .json(&listener_data)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Listener updated successfully.");
                    } else {
                        println!(
                            "Failed to update listener: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::DeleteListener { listener_id } => {
            match client
                .delete(&format!("{}/listeners/{}", url, listener_id))
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Listener deleted successfully.");
                    } else {
                        println!(
                            "Failed to delete listener: {}",
                            response.text().unwrap_or_default()
                        );
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Commands, OnlineArgs};
    use mockito::Server;
    use serde_json::json;

    #[test]
    fn test_online_mode_with_default_url() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "/bots" endpoint
        let _mock = server
            .mock("GET", "/bots")
            .with_status(200)
            .with_body(json!([{ "name": "TestBot" }]).to_string())
            .create();

        // Construct the test arguments
        let args = OnlineArgs {
            url: Some(server.url()), // Use the mock server URL
            command: Some(Commands::ListBots),
        };

        // Run the online_mode function with the test arguments
        online_mode(args);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_add_bot() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "POST /bots" endpoint
        let _mock = server
            .mock("POST", "/bots")
            .with_status(201)
            .with_body("Bot added successfully.")
            .create();

        // Create the command to test
        let command = Commands::AddBot {
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: Some("api-key".to_string()),
            api_secret: Some("api-secret".to_string()),
            rest_endpoint: Some("http://rest-endpoint".to_string()),
            rpc_endpoint: Some("http://rpc-endpoint".to_string()),
            webhook_secret: Some("webhook-secret".to_string()),
            trading_fee: Some(0.1),
            private_key: Some("private-key".to_string()),
            contract_address: Some("contract-address".to_string()),
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_list_bots() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "GET /bots" endpoint
        let _mock = server
            .mock("GET", "/bots")
            .with_status(200)
            .with_body(json!([{ "name": "TestBot" }]).to_string())
            .create();

        // Create the command to test
        let command = Commands::ListBots;

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_get_bot() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "GET /bots/{bot_id}" endpoint
        let _mock = server
            .mock("GET", "/bots/123")
            .with_status(200)
            .with_body(json!({ "name": "TestBot" }).to_string())
            .create();

        // Create the command to test
        let command = Commands::GetBot {
            bot_id: "123".to_string(),
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_update_bot() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "PUT /bots/{bot_id}" endpoint
        let _mock = server
            .mock("PUT", "/bots/123")
            .with_status(200)
            .with_body("Bot updated successfully.")
            .create();

        // Create the command to test
        let command = Commands::UpdateBot {
            bot_id: "123".to_string(),
            name: Some("UpdatedBot".to_string()),
            exchange: None,
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.2),
            private_key: None,
            contract_address: None,
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_delete_bot() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "DELETE /bots/{bot_id}" endpoint
        let _mock = server
            .mock("DELETE", "/bots/123")
            .with_status(200)
            .with_body("Bot deleted successfully.")
            .create();

        // Create the command to test
        let command = Commands::DeleteBot {
            bot_id: "123".to_string(),
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_add_listener() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "POST /listeners" endpoint
        let _mock = server
            .mock("POST", "/listeners")
            .with_status(201)
            .with_body("Listener added successfully.")
            .create();

        // Create the command to test
        let command = Commands::AddListener {
            bot_id: "123".to_string(),
            service: "TestService".to_string(),
            secret: Some("secret".to_string()),
            msg: Some(json!({"key": "value"}).to_string()),
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_list_listeners() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "GET /bots/{bot_id}/listeners" endpoint
        let _mock = server
            .mock("GET", "/bots/123/listeners")
            .with_status(200)
            .with_body(json!([{ "service": "TestService" }]).to_string())
            .create();

        // Create the command to test
        let command = Commands::ListListeners {
            bot_id: "123".to_string(),
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }

    #[test]
    fn test_handle_online_mode_delete_listener() {
        // Start a mock server
        let mut server = Server::new();

        // Create a mock for the "DELETE /listeners/{listener_id}" endpoint
        let _mock = server
            .mock("DELETE", "/listeners/listener123")
            .with_status(200)
            .with_body("Listener deleted successfully.")
            .create();

        // Create the command to test
        let command = Commands::DeleteListener {
            listener_id: "listener123".to_string(),
        };

        // Run the handle_online_mode function
        handle_online_mode(&server.url(), command);

        // Verify the mock was called
        _mock.assert();
    }
}
