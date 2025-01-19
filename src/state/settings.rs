use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApiConfig {
    pub port: u16,
    pub bind: String,
    pub state: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClientConfig {
    pub enabled: bool,
    pub port: u16,
    pub bind: String,
    pub path: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WebhookConfig {
    pub enabled: bool,
    pub port: u16,
    pub bind: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OnlineConfig {
    pub url: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OfflineConfig {
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub client: ClientConfig,
    pub webhook: WebhookConfig,
    #[allow(dead_code)]
    pub online: OnlineConfig,
    #[allow(dead_code)]
    pub offline: OfflineConfig,
}

impl AppConfig {
    /// Load configuration with optional environment-based overrides
    pub fn load<P: AsRef<Path>>(file_path: Option<P>) -> Result<Self, config::ConfigError> {
        let config_path = file_path
            .map(|p| p.as_ref().to_path_buf()) // If a path is provided, use it
            .or_else(|| std::env::var("CONFIG_FILE").ok().map(PathBuf::from)) // Check `CONFIG_PATH` environment variable
            .unwrap_or_else(|| PathBuf::from("config.toml")); // Default to "config.toml" in the current directory
        log::info!("Loading configuration from: {}", config_path.display());
        // Load and deserialize the configuration file
        Config::builder()
            .add_source(File::from(config_path).required(false)) // Optional config file
            .build()?
            .try_deserialize::<AppConfig>()
    }
    pub fn save<P: AsRef<Path>>(&self, file_path: Option<P>) -> Result<(), config::ConfigError> {
        let save_path = file_path
            .map(|p| p.as_ref().to_path_buf()) // If a path is provided, use it
            .or_else(|| std::env::var("CONFIG_FILE").ok().map(PathBuf::from)) // Check `CONFIG_PATH` environment variable
            .unwrap_or_else(|| PathBuf::from("config.toml")); // Default to "config.toml" in the current directory

        // Serialize the configuration to a string
        let serialized = toml::to_string_pretty(self)
            .map_err(|e| config::ConfigError::Message(format!("Serialization error: {}", e)))?;

        // Write the serialized string to the file
        std::fs::write(&save_path, serialized)
            .map_err(|e| config::ConfigError::Message(format!("File write error: {}", e)))?;

        log::info!("Saved configuration to: {}", save_path.display());

        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                port: 7762,
                bind: "127.0.0.1".to_string(),
                state: "state.json".to_string(),
            },
            client: ClientConfig {
                enabled: true,
                port: 7763,
                bind: "0.0.0.0".to_string(),
                path: "src/webui/dist".to_string(),
            },
            webhook: WebhookConfig {
                enabled: true,
                port: 7763,
                bind: "0.0.0.0".to_string(),
            },
            online: OnlineConfig {
                url: "http://localhost:7762".to_string(),
            },
            offline: OfflineConfig {
                state: "state.json".to_string(),
            },
        }
    }
}
