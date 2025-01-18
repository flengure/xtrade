pub use crate::bot::model::{Bot, Listener};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotView {
    pub bot_id: String,
    pub name: String,
    pub exchange: String,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub api_secret: Option<String>,
    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub webhook_secret: Option<String>,
    pub trading_fee: Option<f64>,
    #[allow(dead_code)]
    #[serde(skip_serializing)]
    pub private_key: Option<String>,
    pub contract_address: Option<String>,
    pub listeners: HashMap<String, Listener>,
}

impl fmt::Display for BotView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bot ID: {}\nName: {}\nExchange: {}\nREST Endpoint: {:?}\nRPC Endpoint: {:?}\nTrading Fee: {:?}\nContract Address: {:?}\nListeners: {}",
            self.bot_id,
            self.name,
            self.exchange,
            self.rest_endpoint,
            self.rpc_endpoint,
            self.trading_fee,
            self.contract_address,
            self.listeners.len() // Display the number of listeners instead of their details
        )
    }
}

impl From<Bot> for BotView {
    fn from(args: Bot) -> Self {
        BotView {
            bot_id: args.bot_id,
            name: args.name,
            exchange: args.exchange,
            api_key: args.api_key,
            api_secret: args.api_secret,
            rest_endpoint: args.rest_endpoint,
            rpc_endpoint: args.rpc_endpoint,
            webhook_secret: args.webhook_secret,
            trading_fee: args.trading_fee,
            private_key: args.private_key,
            contract_address: args.contract_address,
            listeners: args.listeners,
        }
    }
}
