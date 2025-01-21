use super::AppConfig;
use crate::bot::model::Bot;
use crate::errors::ServerError;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    #[serde(default)]
    pub bots: HashMap<String, Bot>,
    #[serde(default)]
    pub config: AppConfig, // Running configuration
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            bots: HashMap::new(),
            config: AppConfig::default(),
        }
    }
}

impl AppState {
    /// Loads the application state from a JSON file or creates a new blank file if it doesn't exist.
    pub fn load<P: AsRef<Path>>(app_config: AppConfig) -> Result<AppState, ServerError> {
        // Determine the file path
        let state_file = app_config.clone().api_server.state_file_path;

        // Attempt to read the state file, or create it if it doesn't exist
        let state_content = match fs::read_to_string(&state_file) {
            Ok(content) => content,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                info!(
                    "State file not found. Creating a new blank file at: {:?}",
                    state_file
                );
                // Create and write an empty JSON object in one step
                fs::write(&state_file, b"{}").map_err(|e| ServerError::FileWriteError {
                    source: e,
                    path: state_file.clone(),
                })?;
                "{}".to_string()
            }
            Err(e) => {
                return Err(ServerError::FileReadError {
                    source: e,
                    path: state_file.clone(),
                });
            }
        };

        // Test writeability of the file
        if OpenOptions::new().write(true).open(&state_file).is_err() {
            return Err(ServerError::FileWriteError {
                source: std::io::Error::new(ErrorKind::PermissionDenied, "File not writable"),
                path: state_file.clone(),
            });
        }

        // Deserialize the JSON content into `AppState`
        let mut state: AppState =
            serde_json::from_str(&state_content).map_err(ServerError::JsonParseError)?;

        // Update the loaded state with AppConfig and file path
        state.config = app_config;

        info!("State loaded successfully from: {:?}", state_file);
        Ok(state)
    }

    /// Saves the current state to a JSON file.
    pub fn save<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<(), ServerError> {
        let state_file = file_path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(|| Some(self.config.api_server.state_file_path.clone()))
            .ok_or(ServerError::NoFilePathProvided)?;

        // Serialize the AppState to JSON
        let state_json = serde_json::to_string_pretty(self).map_err(ServerError::JsonParseError)?;

        // Save AppConfig
        self.config.save::<&Path>(None).map_err(|e| {
            error!("Failed to save AppConfig: {}", e);
            ServerError::ConfigError(e)
        })?;

        // Write the JSON to the file
        fs::write(&state_file, state_json).map_err(|e| ServerError::FileWriteError {
            source: e,
            path: state_file.clone(),
        })?;

        info!("State saved successfully to file: {:?}", state_file);
        Ok(())
    }
}
