// src/cli/online.rs
use crate::api::{
    add_bot, add_listener, delete_bot, delete_listener, fetch_bot, fetch_bots, fetch_listener,
    fetch_listeners, update_bot, update_listener,
};
use crate::cli::{Commands, OnlineArgs};
use crate::utils::config::AppConfig;
use log::info;
use reqwest::blocking::Client;

/// Run the application in online mode
pub fn online_mode(args: OnlineArgs) {
    // Load configuration using AppConfig
    let config = AppConfig::load(None).unwrap_or_else(|err| {
        log::warn!(
            "Failed to load configuration file: {}. Using defaults.",
            err
        );
        AppConfig::default()
    });
    log::debug!("Loaded configuration: {:?}", config);

    // Use the `online` section of the config for URL fallback
    let server_url = args.url.unwrap_or_else(|| config.online.url);

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
            match add_bot(
                &client,
                &url,
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
            ) {
                Ok(_) => println!("Bot added successfully."),
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::ListBots => match fetch_bots(&client, &url) {
            Ok(bots) => println!("Bots: {}", bots),
            Err(err) => println!("Failed to fetch bots: {}", err),
        },

        Commands::GetBot { bot_id } => match fetch_bot(&client, &url, &bot_id) {
            Ok(bot) => println!("Bot: {}", bot),
            Err(err) => println!("Failed to fetch bot: {}", err),
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
            match update_bot(
                &client,
                &url,
                &bot_id,
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
            ) {
                Ok(_) => println!("Bot updated successfully."),
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::DeleteBot { bot_id } => match delete_bot(&client, &url, &bot_id) {
            Ok(_) => println!("Bot deleted successfully."),
            Err(err) => println!("Error: {}", err),
        },

        Commands::AddListener {
            bot_id,
            service,
            secret,
            msg,
        } => match add_listener(&client, &url, bot_id, service, secret, msg) {
            Ok(_) => println!("Listener added successfully."),
            Err(err) => println!("Error: {}", err),
        },

        Commands::ListListeners { bot_id } => match fetch_listeners(&client, &url, &bot_id) {
            Ok(listeners) => println!("Listeners: {}", listeners),
            Err(err) => println!("Failed to fetch listeners: {}", err),
        },

        Commands::GetListener { listener_id } => {
            match fetch_listener(&client, &url, &listener_id) {
                Ok(listener) => println!("Listener: {}", listener),
                Err(err) => println!("Error: {}", err),
            }
        }

        Commands::UpdateListener {
            listener_id,
            service,
            secret,
            msg,
        } => match update_listener(&client, &url, &listener_id, service, secret, msg) {
            Ok(_) => println!("Listener updated successfully."),
            Err(err) => println!("Error: {}", err),
        },

        Commands::DeleteListener { listener_id } => {
            match delete_listener(&client, &url, &listener_id) {
                Ok(_) => println!("Listener deleted successfully."),
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
