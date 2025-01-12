use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub webhook: WebhookConfig,
    pub bots: Vec<BotConfig>,
    pub listeners: Vec<ListenerConfig>, // Listeners configuration
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct WebhookConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub bot_id: String, // Unique identifier for the bot
    pub name: String,
    pub exchange: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,
    pub webhook_secret: Option<String>,
    pub trading_fee: f64,
    pub private_key: Option<String>,
    pub contract_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListenerConfig {
    pub name: String,
    pub service: String, // Service type (e.g., "TradingView")
    pub target: String,  // Maps to the bot_id
    pub secret: String,  // Secret for validation
    pub msg: String,     // Message template
}
