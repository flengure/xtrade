//use crate::models::Listener;
pub use crate::bot::model::{Bot, Listener};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BotUpdateArgs {
    pub name: Option<String>,
    pub exchange: Option<String>,
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

impl BotUpdateArgs {
    /// Creates a new `BotUpdateArgs` instance with all fields as `None`.
    pub fn new() -> Self {
        Self {
            name: None,
            exchange: None,
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: None,
            private_key: None,
            contract_address: None,
            listeners: HashMap::new(),
        }
    }

    /// Fluent builder-style methods to set each field.

    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn exchange(mut self, exchange: Option<String>) -> Self {
        self.exchange = exchange;
        self
    }

    pub fn api_key(mut self, api_key: Option<String>) -> Self {
        self.api_key = api_key;
        self
    }

    pub fn api_secret(mut self, api_secret: Option<String>) -> Self {
        self.api_secret = api_secret;
        self
    }

    pub fn rest_endpoint(mut self, rest_endpoint: Option<String>) -> Self {
        self.rest_endpoint = rest_endpoint;
        self
    }

    pub fn rpc_endpoint(mut self, rpc_endpoint: Option<String>) -> Self {
        self.rpc_endpoint = rpc_endpoint;
        self
    }

    pub fn webhook_secret(mut self, webhook_secret: Option<String>) -> Self {
        self.webhook_secret = webhook_secret;
        self
    }

    pub fn trading_fee(mut self, trading_fee: Option<f64>) -> Self {
        self.trading_fee = trading_fee;
        self
    }

    pub fn private_key(mut self, private_key: Option<String>) -> Self {
        self.private_key = private_key;
        self
    }

    pub fn contract_address(mut self, contract_address: Option<String>) -> Self {
        self.contract_address = contract_address;
        self
    }

    #[allow(dead_code)]
    pub fn listeners(mut self, listeners: HashMap<String, Listener>) -> Self {
        self.listeners = listeners;
        self
    }

    /// Applies the updates to an existing `Bot` instance.
    pub fn apply(&self, bot: &mut Bot) {
        if let Some(name) = &self.name {
            bot.name = name.clone();
        }
        if let Some(exchange) = &self.exchange {
            bot.exchange = exchange.clone();
        }
        if let Some(api_key) = &self.api_key {
            bot.api_key = Some(api_key.clone());
        }
        if let Some(api_secret) = &self.api_secret {
            bot.api_secret = Some(api_secret.clone());
        }
        if let Some(rest_endpoint) = &self.rest_endpoint {
            bot.rest_endpoint = Some(rest_endpoint.clone());
        }
        if let Some(rpc_endpoint) = &self.rpc_endpoint {
            bot.rpc_endpoint = Some(rpc_endpoint.clone());
        }
        if let Some(webhook_secret) = &self.webhook_secret {
            bot.webhook_secret = Some(webhook_secret.clone());
        }
        if let Some(trading_fee) = &self.trading_fee {
            bot.trading_fee = Some(*trading_fee);
        }
        if let Some(private_key) = &self.private_key {
            bot.private_key = Some(private_key.clone());
        }
        if let Some(contract_address) = &self.contract_address {
            bot.contract_address = Some(contract_address.clone());
        }
        // Replace the entire listeners map if specified
        if !self.listeners.is_empty() {
            bot.listeners = self.listeners.clone();
        }
    }
}
