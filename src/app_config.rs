use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ApiServerConfig {
    /// Port number for the API server
    pub port: u16,
    /// Address to bind the API server
    pub bind_address: String,
    /// File path for the application state
    pub state_file: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WebhookServerConfig {
    /// Port number for the Webhook Server
    pub port: u16,
    /// Address to bind the Webhook Server
    pub bind_address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WebClientConfig {
    /// Whether the Web Client is enabled
    pub enable: bool,
    /// Port number for the Web Client
    pub port: u16,
    /// Address to bind the Web Client
    pub bind_address: String,
    /// Path to the Web Client's static files
    pub static_files: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RemoteCliConfig {
    /// URL of the remote server for online mode
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct LocalCliConfig {
    /// File path for the local state file (offline mode)
    pub state_file: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AppConfig {
    /// Configuration for the API Server
    pub api_server: ApiServerConfig,
    /// Configuration for the Webhook Server
    pub webhook_server: WebhookServerConfig,
    /// Configuration for the Web Client
    pub web_client: WebClientConfig,
    /// Configuration for online mode
    //#[allow(dead_code)]
    pub remote_cli: RemoteCliConfig,
    /// Configuration for offline mode
    //#[allow(dead_code)]
    pub local_cli: LocalCliConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_server: ApiServerConfig {
                port: 7762,
                bind_address: "127.0.0.1".to_string(),
                state_file: PathBuf::from("state.json"),
            },
            webhook_server: WebhookServerConfig {
                port: 7763,
                bind_address: "0.0.0.0".to_string(),
            },
            web_client: WebClientConfig {
                enable: true,
                port: 7764,
                bind_address: "0.0.0.0".to_string(),
                static_files: PathBuf::from("src/webui/dist"),
            },
            remote_cli: RemoteCliConfig {
                url: "http://localhost:7762".to_string(),
            },
            local_cli: LocalCliConfig {
                state_file: PathBuf::from("state.json"),
            },
        }
    }
}

impl AppConfig {
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

    /// Load configuration with optional environment-based overrides
    pub fn load<P: AsRef<Path>>(file_path: Option<P>) -> Result<Self, config::ConfigError> {
        let config_path = file_path
            .map(|p| p.as_ref().to_path_buf())
            .or_else(|| std::env::var("CONFIG_FILE").ok().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        log::info!("Loading configuration from: {}", config_path.display());

        // Try loading the configuration file
        let result = Config::builder()
            .add_source(File::from(config_path.clone()).required(false)) // Optional config file
            .build()
            .and_then(|cfg| cfg.try_deserialize::<AppConfig>());

        match result {
            Ok(config) => Ok(config),
            Err(_) => {
                log::warn!(
                    "Failed to load configuration from {}. Falling back to defaults.",
                    config_path.display()
                );
                let default_config = AppConfig::default();

                // Attempt to save defaults and let `save` handle errors/logging
                let _ = default_config.save(Some(&config_path));

                Ok(default_config)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_appconfig_default() {
        let default_config = AppConfig::default();

        assert_eq!(default_config.api_server.port, 7762);
        assert_eq!(default_config.api_server.bind_address, "127.0.0.1");
        assert_eq!(
            default_config.api_server.state_file,
            PathBuf::from("state.json")
        );

        assert_eq!(default_config.webhook_server.port, 7764);

        assert!(default_config.web_client.enable);
        assert_eq!(default_config.web_client.port, 7763);
        assert_eq!(
            default_config.web_client.static_files,
            PathBuf::from("src/webui/dist")
        );

        assert_eq!(default_config.remote_cli.url, "http://localhost:7762");
        assert_eq!(
            default_config.local_cli.state_file,
            PathBuf::from("state.json")
        );
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let original_config = AppConfig::default();

        // Save the configuration
        original_config.save(Some(&config_path)).unwrap();

        // Verify the file exists
        assert!(config_path.exists());

        // Load the configuration
        let loaded_config = AppConfig::load(Some(&config_path)).unwrap();

        // Verify the loaded config matches the original
        assert_eq!(
            original_config.api_server.port,
            loaded_config.api_server.port
        );
        assert_eq!(
            original_config.api_server.bind_address,
            loaded_config.api_server.bind_address
        );
        assert_eq!(
            original_config.api_server.state_file,
            loaded_config.api_server.state_file
        );
    }

    #[test]
    fn test_load_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("nonexistent_config.toml");

        // Ensure the file doesn't exist
        assert!(!config_path.exists());

        // Load configuration (should create a default file)
        let config = AppConfig::load(Some(&config_path)).unwrap();

        // Verify the defaults are used
        assert_eq!(config.api_server.port, 7762);
        assert_eq!(config.api_server.bind_address, "127.0.0.1");

        // Verify the file is created
        assert!(config_path.exists());
    }

    #[test]
    fn test_invalid_config_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("invalid_config.toml");

        // Write invalid content to the file
        fs::write(&config_path, "invalid_toml_content").unwrap();

        // Load configuration (should fallback to defaults)
        let config = AppConfig::load(Some(&config_path)).unwrap();

        // Verify the defaults are used
        assert_eq!(config.api_server.port, 7762);
        assert_eq!(config.api_server.bind_address, "127.0.0.1");
    }
}
