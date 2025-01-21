//! # Application State Management
//!
//! This module provides utilities for managing the application state of the xTrade system.
//! The state is represented by the [`AppState`] struct, which stores information about bots and
//! application configuration. The state can be persisted to and loaded from a JSON file.
//!
//! ## Key Features
//! - **Loading State**: Loads the state from a specified file or creates a new file with default
//!   values if the file is missing.
//! - **Saving State**: Saves the current state to a file, ensuring the state is persisted across
//!   restarts.
//! - **Configuration Integration**: The state integrates with [`AppConfig`] to manage runtime
//!   settings.
//!
//! ## Limitations
//! - **In-Memory Storage**: The current implementation uses an in-memory [`HashMap`] to store bots.
//!   This may become a performance bottleneck or consume significant memory if the number of bots
//!   grows large.
//! - **Scalability**: For large-scale deployments, consider transitioning to a database or another
//!   persistent storage mechanism to handle scalability and performance concerns.
//!
//! ## Usage
//! ```rust
//! use crate::state::AppState;
//! use crate::state::AppConfig;
//!
//! let config = AppConfig::default();
//! let state = AppState::load(config).expect("Failed to load application state");
//!
//! state.save(None).expect("Failed to save application state");
//! ```
//!
//! ## Testing
//! - Unit tests are provided to ensure correctness and robustness, including scenarios for missing,
//!   unwritable, and existing state files.
//! - Tests leverage the [`tempfile`] crate to create isolated temporary directories during testing.
//!
//! ## Future Improvements
//! - Add database support for storing and querying bots efficiently.
//! - Implement an event-based state synchronization mechanism for distributed systems.
use super::AppConfig;
use crate::bot::model::Bot;
use crate::errors::ServerError;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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
    pub fn load(app_config: AppConfig) -> Result<AppState, ServerError> {
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
                // Create and test writeability in one step
                fs::write(&state_file, b"{}")
                    .and_then(|_| OpenOptions::new().write(true).open(&state_file))
                    .map_err(|e| ServerError::FileWriteError {
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

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::bot::model::Bot;
    use crate::errors::ServerError;
    use crate::state::settings::ApiServerConfig;
    use crate::state::{AppConfig, AppState};
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    use tempfile::tempdir;

    fn create_test_config(state_file: &Path) -> AppConfig {
        AppConfig {
            api_server: ApiServerConfig {
                port: 7762,
                bind_address: "127.0.0.1".to_string(),
                state_file_path: state_file.to_path_buf(),
            },
            ..AppConfig::default()
        }
    }

    #[test]
    fn test_load_creates_blank_state_file_if_missing() {
        let temp_dir = tempdir().unwrap();
        let state_file = temp_dir.path().join("missing_state.json");
        let config = create_test_config(&state_file);

        // Ensure the state file does not exist
        assert!(!state_file.exists());

        // Load AppState, which should create the state file
        let app_state = AppState::load(config.clone()).unwrap();

        // Check if the file was created
        assert!(state_file.exists());

        // Check if the file contains an empty JSON object
        let content = fs::read_to_string(&state_file).unwrap();
        assert_eq!(content, "{}");

        // Check if the loaded state is the default state
        assert_eq!(app_state, AppState::default());
    }

    #[test]
    fn test_load_existing_state_file() {
        let temp_dir = tempdir().unwrap();
        let state_file = temp_dir.path().join("existing_state.json");
        let config = create_test_config(&state_file);

        // Create a sample state file
        let sample_state = r#"{ "bots": { "bot1": { "name": "Bot 1", "exchange": "Test" } } }"#;
        fs::write(&state_file, sample_state).unwrap();

        // Load AppState
        let app_state = AppState::load(config.clone()).unwrap();

        // Check if the loaded state matches the file content
        assert!(app_state.bots.contains_key("bot1"));
    }

    #[test]
    fn test_save_state_to_file() {
        let temp_dir = tempdir().unwrap();
        let state_file = temp_dir.path().join("saved_state.json");
        //let config = create_test_config(&state_file);

        // Create an AppState
        let mut app_state = AppState::default();
        app_state.bots.insert(
            "bot1".to_string(),
            Bot {
                name: "Test Bot".to_string(),
                exchange: "Test Exchange".to_string(),
                ..Default::default()
            },
        );

        // Save the state
        app_state.save::<&Path>(None).unwrap();

        // Check if the state file exists
        assert!(state_file.exists());

        // Check if the content matches the saved state
        let content = fs::read_to_string(&state_file).unwrap();
        assert!(content.contains("Test Bot"));
    }

    #[test]
    fn test_load_fails_if_file_not_writable() {
        let temp_dir = tempdir().unwrap();
        let state_file = temp_dir.path().join("unwritable_state.json");
        let config = create_test_config(&state_file);

        // Create the file and make it read-only
        fs::write(&state_file, "{}").unwrap();
        let mut permissions = fs::metadata(&state_file).unwrap().permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&state_file, permissions).unwrap();

        // Attempt to load the state, which should fail due to write permissions
        let result = AppState::load(config);
        assert!(result.is_err());
        if let Err(ServerError::FileWriteError { source, path }) = result {
            assert_eq!(path, state_file);
            assert_eq!(source.kind(), ErrorKind::PermissionDenied);
        } else {
            panic!("Expected FileWriteError");
        }
    }
}
