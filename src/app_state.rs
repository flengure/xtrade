// app_state.rs
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
