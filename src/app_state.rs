// src/app_state.rs
use crate::models::{Bot, Listener};
use crate::services::{bots, listeners};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub bots: HashMap<String, Bot>, // Bots now include their own listeners
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            bots: HashMap::new(),
        }
    }
}
impl AppState {
    /// Load state from a JSON file. If the file doesn't exist, return an error.
    pub fn load_state_from_file(file_path: &str) -> Result<AppState, String> {
        match std::fs::read_to_string(file_path) {
            Ok(state_content) => match serde_json::from_str::<AppState>(&state_content) {
                Ok(state) => {
                    log::info!("Loaded state from file: {}", file_path);
                    Ok(state)
                }
                Err(err) => {
                    log::error!("Failed to parse state file {}: {}", file_path, err);
                    Err(format!("Failed to parse state file: {}", err))
                }
            },
            Err(err) => {
                log::warn!(
                    "No state file found or failed to read. Starting with an empty state. Error: {}",
                    err
                );
                Err(format!("Failed to read state file: {}", err))
            }
        }
    }

    /// Save the current state to a JSON file.
    pub fn save_state_to_file(&self, file_path: &str) {
        if let Ok(state_content) = serde_json::to_string_pretty(self) {
            if let Err(e) = std::fs::write(file_path, state_content) {
                log::error!("Failed to save state to file {}: {}", file_path, e);
            } else {
                log::info!("State saved to file: {}", file_path);
            }
        } else {
            log::error!("Failed to serialize state for saving.");
        }
    }

    /// Add a bot
    pub fn add_bot(&mut self, bot: Bot) -> String {
        bots::create_bot(&mut self.bots, bot)
    }

    /// List all bots
    pub fn list_bots(&self) -> Vec<&Bot> {
        bots::list_bots(&self.bots)
    }

    /// Get a bot by ID
    pub fn get_bot(&self, bot_id: &str) -> Option<&Bot> {
        bots::get_bot(&self.bots, bot_id)
    }

    /// Update a bot
    pub fn update_bot(&mut self, bot_id: &str, updated_bot: Bot) -> Result<(), String> {
        bots::update_bot(&mut self.bots, bot_id, updated_bot)
    }

    /// Delete a bot
    pub fn delete_bot(&mut self, bot_id: &str) -> Result<(), String> {
        bots::delete_bot(&mut self.bots, bot_id)
    }

    /// Add a listener
    pub fn add_listener(&mut self, bot_id: &str, listener: Listener) -> Result<String, String> {
        listeners::add_listener(&mut self.bots, bot_id, listener)
    }

    /// List all listeners for a given bot ID
    pub fn list_listeners(&self, bot_id: &str) -> Result<Vec<&Listener>, String> {
        listeners::list_listeners(&self.bots, bot_id)
    }

    /// Get a listener by its ID
    pub fn get_listener(&self, listener_id: &str) -> Option<&Listener> {
        listeners::get_listener(&self.bots, listener_id)
    }

    /// Update a listener by its ID
    pub fn update_listener(
        &mut self,
        listener_id: &str,
        updated_listener: Listener,
    ) -> Result<(), String> {
        listeners::update_listener(&mut self.bots, listener_id, updated_listener)
    }

    /// Delete a listener by its ID
    pub fn delete_listener(&mut self, listener_id: &str) -> Result<(), String> {
        listeners::delete_listener(&mut self.bots, listener_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Bot, Listener};
    use serde_json::json;
    //use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    /// Helper function to create a mock bot
    fn mock_bot(bot_id: &str) -> Bot {
        Bot {
            bot_id: bot_id.to_string(),
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: Some("api_key".to_string()),
            api_secret: Some("api_secret".to_string()),
            rest_endpoint: Some("http://rest_endpoint".to_string()),
            rpc_endpoint: Some("http://rpc_endpoint".to_string()),
            webhook_secret: Some("webhook_secret".to_string()),
            trading_fee: Some(0.1),
            private_key: Some("private_key".to_string()),
            contract_address: Some("contract_address".to_string()),
            listeners: Vec::new(),
        }
    }

    /// Helper function to create a mock listener
    fn mock_listener(listener_id: &str) -> Listener {
        Listener {
            listener_id: listener_id.to_string(),
            service: "TestService".to_string(),
            secret: "secret123".to_string(),
            msg: json!({"key": "value"}),
        }
    }

    #[test]
    fn test_add_and_list_bots() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let bots = app_state.list_bots();
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0].bot_id, "bot123");
    }

    #[test]
    fn test_get_bot() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let bot = app_state.get_bot("bot123");
        assert!(bot.is_some());
        assert_eq!(bot.unwrap().bot_id, "bot123");
    }

    #[test]
    fn test_update_bot() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let updated_bot = Bot {
            bot_id: "bot123".to_string(),
            name: "UpdatedBot".to_string(),
            ..mock_bot("bot123")
        };

        let result = app_state.update_bot("bot123", updated_bot.clone());
        assert!(result.is_ok());

        let bot = app_state.get_bot("bot123").unwrap();
        assert_eq!(bot.name, "UpdatedBot");
    }

    #[test]
    fn test_delete_bot() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let result = app_state.delete_bot("bot123");
        assert!(result.is_ok());
        assert!(app_state.get_bot("bot123").is_none());
    }

    #[test]
    fn test_add_and_list_listeners() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let listener = mock_listener("listener123");
        let result = app_state.add_listener("bot123", listener.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "listener123");

        let listeners = app_state.list_listeners("bot123").unwrap();
        assert_eq!(listeners.len(), 1);
        assert_eq!(listeners[0].listener_id, "listener123");
    }

    #[test]
    fn test_get_listener() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let listener = mock_listener("listener123");
        app_state.add_listener("bot123", listener.clone()).unwrap();

        let fetched_listener = app_state.get_listener("listener123");
        assert!(fetched_listener.is_some());
        assert_eq!(fetched_listener.unwrap().listener_id, "listener123");
    }

    #[test]
    fn test_update_listener() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let listener = mock_listener("listener123");
        app_state.add_listener("bot123", listener.clone()).unwrap();

        let updated_listener = Listener {
            listener_id: "listener123".to_string(),
            service: "UpdatedService".to_string(),
            ..listener
        };

        let result = app_state.update_listener("listener123", updated_listener.clone());
        assert!(result.is_ok());

        let listener = app_state.get_listener("listener123").unwrap();
        assert_eq!(listener.service, "UpdatedService");
    }

    #[test]
    fn test_delete_listener() {
        let mut app_state = AppState::default();
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        let listener = mock_listener("listener123");
        app_state.add_listener("bot123", listener.clone()).unwrap();

        let result = app_state.delete_listener("listener123");
        assert!(result.is_ok());

        let listeners = app_state.list_listeners("bot123").unwrap();
        assert!(listeners.is_empty());
    }

    #[test]
    fn test_load_and_save_state() {
        let file_path = "test_state.json";
        let mut app_state = AppState::default();

        // Add a bot
        let bot = mock_bot("bot123");
        app_state.add_bot(bot);

        // Save state to file
        app_state.save_state_to_file(file_path);
        assert!(Path::new(file_path).exists());

        // Load state from file
        let loaded_state = AppState::load_state_from_file(file_path).unwrap();
        assert_eq!(loaded_state.bots.len(), 1);
        assert_eq!(loaded_state.bots["bot123"].bot_id, "bot123");

        // Clean up
        fs::remove_file(file_path).unwrap();
    }
}
