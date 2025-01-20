//use crate::models::Listener;
use crate::bot::model::{Bot, Listener};
use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotUpdateArgs {
    #[arg(long)]
    pub bot_id: String,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub exchange: Option<String>,
    #[arg(long)]
    pub api_key: Option<String>,
    #[arg(long)]
    pub api_secret: Option<String>,
    #[arg(long)]
    pub rest_endpoint: Option<String>,
    #[arg(long)]
    pub rpc_endpoint: Option<String>,
    #[arg(long)]
    pub webhook_secret: Option<String>,
    #[arg(long)]
    pub trading_fee: Option<f64>,
    #[arg(long)]
    pub private_key: Option<String>,
    #[arg(long)]
    pub contract_address: Option<String>,
    #[arg(skip)]
    pub listeners: HashMap<String, Listener>,
}

impl BotUpdateArgs {
    /// Creates a new `BotUpdateArgs` instance with all fields as `None`.
    #[allow(dead_code)]
    pub fn new(bot_id: &str) -> Self {
        Self {
            bot_id: bot_id.to_string(),
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
    #[allow(dead_code)]
    pub fn bot_id(mut self, bot_id: &str) -> Self {
        self.bot_id = bot_id.to_string();
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
