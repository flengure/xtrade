use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    pub port: u16,
    pub bind: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct WebConfig {
    pub enabled: bool,
    pub port: u16,
    pub bind: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct OnlineConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct OfflineConfig {
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub web: WebConfig,
    pub online: OnlineConfig,
    pub offline: OfflineConfig,
}

impl AppConfig {
    /// Load configuration with optional environment-based overrides
    pub fn load(path: Option<&str>) -> Result<Self, config::ConfigError> {
        let config_path = path
            .map(|p| p.to_string()) // If a path is provided, use it
            .or_else(|| std::env::var("CONFIG_PATH").ok()) // Check `CONFIG_PATH` environment variable
            .unwrap_or_else(|| "config.toml".to_string()); // Default to "config.toml" in the current directory

        log::info!("Loading configuration from: {}", config_path);

        let result = Config::builder()
            .add_source(File::with_name(&config_path).required(false)) // Optional config file
            .build()
            .and_then(|cfg| {
                log::info!("Configuration successfully loaded: {:#?}", cfg);
                cfg.try_deserialize::<AppConfig>()
            });

        if let Err(ref err) = result {
            log::error!("Error loading configuration: {:?}", err);
        }

        result
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                port: 7762,
                bind: "127.0.0.1".to_string(),
                state: "state.json".to_string(),
            },
            web: WebConfig {
                enabled: true,
                port: 7763,
                bind: "0.0.0.0".to_string(),
                path: "src/webui/dist".to_string(),
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
