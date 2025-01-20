use super::AppConfig;
use crate::bot::model::Bot;
use crate::errors::ServerError;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub bots: HashMap<String, Bot>,
    pub file: Option<PathBuf>,
    #[serde(default)]
    pub config: AppConfig, // Running configuration
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            bots: HashMap::new(),
            file: Some(PathBuf::from("state.json")),
            config: AppConfig::default(),
        }
    }
}

impl AppState {
    /// Create a default AppState wrapped in Arc<Mutex>
    #[allow(dead_code)]
    pub fn default_shared() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }

    /// Loads the application state from a JSON file.
    pub fn load<P: AsRef<Path>>(file_path: Option<P>) -> Result<AppState, ServerError> {
        // Determine the file path
        let state_file = file_path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(|| std::env::var("STATE_FILE").ok().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("state.json"));

        info!("Loading state from file: {:?}", &state_file);

        // Load AppConfig
        let app_config = AppConfig::load::<&Path>(None).map_err(|e| {
            error!("Failed to load AppConfig: {}", e);
            ServerError::ConfigError(e)
        })?;

        // Read the state file
        let state_content =
            std::fs::read_to_string(&state_file).map_err(|e| ServerError::FileReadError {
                source: e,
                path: state_file.clone(),
            })?;

        // Deserialize the JSON content into `AppState`
        let mut state: AppState =
            serde_json::from_str(&state_content).map_err(ServerError::JsonParseError)?;

        // Update the loaded state with AppConfig and file path
        state.config = app_config;
        state.file = Some(state_file.clone());

        info!("State loaded successfully from: {:?}", state_file);
        Ok(state)
    }

    /// Saves the current state to a JSON file.
    pub fn save<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<(), ServerError> {
        let state_file = file_path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(|| self.file.clone())
            .ok_or(ServerError::NoFilePathProvided)?;

        //info!("Saving state to file: {:?}", &state_file);

        // Serialize the AppState to JSON
        let state_json = serde_json::to_string_pretty(self).map_err(ServerError::JsonParseError)?;

        // Save AppConfig
        self.config.save::<&Path>(None).map_err(|e| {
            error!("Failed to save AppConfig: {}", e);
            ServerError::ConfigError(e)
        })?;

        // Write the JSON to the file
        std::fs::write(&state_file, state_json).map_err(|e| ServerError::FileWriteError {
            source: e,
            path: state_file.clone(),
        })?;

        info!("State saved successfully to file: {:?}", state_file);
        Ok(())
    }
}
