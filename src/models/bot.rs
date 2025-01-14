use crate::models::Listener;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

/// Represents a trading bot with its configuration and associated listeners.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    pub bot_id: String,
    pub name: String,
    pub exchange: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,
    pub webhook_secret: Option<String>,
    pub trading_fee: Option<f64>,
    pub private_key: Option<String>,
    pub contract_address: Option<String>,
    pub listeners: HashMap<String, Listener>,
}

/// Represents the input structure for creating or updating a bot.
///
/// This struct is used for validating user-provided data before creating or updating a bot.
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BotInsert {
    /// Optional unique identifier for the bot. If not provided, a new UUID will be generated.
    pub bot_id: Option<String>,

    /// Human-readable name for the bot.
    ///
    /// Must be non-empty.
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    /// The exchange where the bot operates (e.g., Binance, Coinbase).
    ///
    /// Must be non-empty.
    #[validate(length(min = 1, message = "Exchange cannot be empty"))]
    pub exchange: String,

    /// API key for the exchange (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub api_key: Option<String>,

    /// API secret for the exchange (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub api_secret: Option<String>,

    /// The REST API endpoint the bot interacts with (optional).
    pub rest_endpoint: Option<String>,

    /// The RPC endpoint for blockchain interactions (optional).
    pub rpc_endpoint: Option<String>,

    /// Webhook secret used for secure webhooks (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub webhook_secret: Option<String>,

    /// Trading fee percentage applied by the exchange (optional).
    pub trading_fee: Option<f64>,

    /// Private key for signing transactions (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub private_key: Option<String>,

    /// Smart contract address the bot interacts with (optional).
    pub contract_address: Option<String>,
}

/// Represents the input structure for creating or updating a bot.
///
/// This struct is used for validating user-provided data before creating or updating a bot.
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BotUpdate {
    pub bot_id: String,
    pub name: Option<String>,
    pub exchange: Option<String>,

    /// API key for the exchange (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub api_key: Option<String>,

    /// API secret for the exchange (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub api_secret: Option<String>,

    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,

    /// Webhook secret used for secure webhooks (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub webhook_secret: Option<String>,

    pub trading_fee: Option<f64>,

    /// Private key for signing transactions (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub private_key: Option<String>,

    /// Smart contract address the bot interacts with (optional).
    pub contract_address: Option<String>,
}

/// Represents the input structure for creating or updating a bot.
///
/// This struct is used for validating user-provided data before creating or updating a bot.
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BotInput {
    /// Optional unique identifier for the bot. If not provided, a new UUID will be generated.
    pub bot_id: Option<String>,

    /// Human-readable name for the bot.
    ///
    /// Must be non-empty.
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    /// The exchange where the bot operates (e.g., Binance, Coinbase).
    ///
    /// Must be non-empty.
    #[validate(length(min = 1, message = "Exchange cannot be empty"))]
    pub exchange: String,

    /// API key for the exchange (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub api_key: Option<String>,

    /// API secret for the exchange (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub api_secret: Option<String>,

    /// The REST API endpoint the bot interacts with (optional).
    pub rest_endpoint: Option<String>,

    /// The RPC endpoint for blockchain interactions (optional).
    pub rpc_endpoint: Option<String>,

    /// Webhook secret used for secure webhooks (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub webhook_secret: Option<String>,

    /// Trading fee percentage applied by the exchange (optional).
    pub trading_fee: Option<f64>,

    /// Private key for signing transactions (optional).
    ///
    /// This is excluded from serialization for security reasons.
    #[serde(skip_serializing)]
    pub private_key: Option<String>,

    /// Smart contract address the bot interacts with (optional).
    pub contract_address: Option<String>,
}
