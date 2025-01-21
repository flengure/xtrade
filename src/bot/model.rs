// src/bot/model.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a trading bot with its configuration and associated listeners.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Listener {
    pub service: String, // Service type (e.g., TradingView)
    pub secret: String,  // Security secret for the webhook
    pub msg: String,     // Change msg to serde_json::Value
}
