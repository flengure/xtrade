// src/state.rs
use crate::bot::model::Bot;
use crate::errors::ServerError;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Represents the application state.
///
/// # Fields
///
/// * `bots` - A collection of trading bots indexed by their unique IDs.
/// * `file` - The file path where the application state is stored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub bots: HashMap<String, Bot>,
    pub file: Option<PathBuf>,
}

impl Default for AppState {
    /// Provides a default implementation for `AppState`.
    ///
    /// # Returns
    ///
    /// A new `AppState` with an empty collection of bots and the state file set to "state.json".
    fn default() -> Self {
        AppState {
            bots: HashMap::new(),
            file: Some(PathBuf::from("state.json")),
        }
    }
}

impl AppState {
    /// Create a default AppState wrapped in Arc<Mutex>
    pub fn default_shared() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }

    /// Loads the application state from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - An optional path to load the state file.
    ///
    /// # Returns
    ///
    /// * `Ok(AppState)` if the state is loaded successfully.
    /// * `Err(LoadError)` if an error occurs during loading.
    pub fn load<P: AsRef<Path>>(file_path: Option<P>) -> Result<AppState, ServerError> {
        // Determine the file path based on the following priority:
        // 1. Use the provided `file_path` if Some.
        // 2. Else, use the `STATE_FILE` environment variable if set.
        // 3. Else, use the `file` field if Some.
        // 4. Else, default to "state.json".
        let path = if let Some(p) = file_path {
            p.as_ref().to_path_buf()
        } else if let Ok(env_path) = std::env::var("STATE_FILE") {
            PathBuf::from(env_path)
        } else if let Some(ref p) = AppState::default().file {
            p.clone()
        } else {
            PathBuf::from("state.json")
        };

        // Attempt to read the state file
        let state_content =
            std::fs::read_to_string(&path).map_err(|e| ServerError::FileReadError {
                source: e,
                path: path.clone(),
            })?;

        // Deserialize the JSON content into `AppState`
        let mut state: AppState =
            serde_json::from_str(&state_content).map_err(ServerError::JsonParseError)?;

        // Update the file path field in the loaded state
        state.file = Some(path.clone());

        info!("State loaded from file: {:?}", &path);
        Ok(state)
    }

    /// Saves the current state to a JSON file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - An optional path to save the state file.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the state is saved successfully.
    /// * `Err(LoadError)` if an error occurs during saving.
    pub fn save<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<(), ServerError> {
        // Determine the file path based on the following priority:
        // 1. Use the provided `file_path` if Some.
        // 2. Else, use the `self.file` field if Some.
        // 3. Else, return an error.
        let path = if let Some(p) = file_path {
            p.as_ref().to_path_buf()
        } else if let Some(ref p) = self.file {
            p.clone()
        } else {
            return Err(ServerError::NoFilePathProvided);
        };

        // Serialize the AppState to JSON
        let state_json = serde_json::to_string_pretty(self).map_err(ServerError::JsonParseError)?;

        // Write the JSON to the file
        std::fs::write(&path, state_json).map_err(|e| ServerError::FileReadError {
            source: e,
            path: path.clone(),
        })?;

        info!("State saved to file: {:?}", &path);
        Ok(())
    }
}
