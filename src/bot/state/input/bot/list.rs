//use crate::models::Listener;
pub use crate::bot::model::{Bot, Listener};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotListArgs {
    pub bot_id: Option<String>,
    pub name: Option<String>,
    pub exchange: Option<String>,
    pub api_key: Option<String>,
    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,
    pub trading_fee: Option<f64>,
    pub private_key: Option<String>,
    pub contract_address: Option<String>,
    pub listeners: HashMap<String, Listener>,
}
impl BotListArgs {
    /// Create a new `BotListArgs` instance with default values
    pub fn new() -> Self {
        Self {
            bot_id: None,
            name: None,
            exchange: None,
            api_key: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            trading_fee: None,
            private_key: None,
            contract_address: None,
            listeners: HashMap::new(), // Initialize as an empty HashMap
        }
    }

    /// Checks whether a `Bot` matches the criteria in `BotListArgs`
    pub fn matches(&self, bot: &Bot) -> bool {
        (self.bot_id.as_ref().map_or(true, |id| &bot.bot_id == id))
            && (self.name.as_ref().map_or(true, |name| &bot.name == name))
            && (self
                .exchange
                .as_ref()
                .map_or(true, |exchange| &bot.exchange == exchange))
            && (self
                .api_key
                .as_ref()
                .map_or(true, |key| bot.api_key.as_ref() == Some(key)))
            && (self.rest_endpoint.as_ref().map_or(true, |endpoint| {
                bot.rest_endpoint.as_ref() == Some(endpoint)
            }))
            && (self
                .rpc_endpoint
                .as_ref()
                .map_or(true, |endpoint| bot.rpc_endpoint.as_ref() == Some(endpoint)))
            && (self
                .trading_fee
                .as_ref()
                .map_or(true, |fee| bot.trading_fee.as_ref() == Some(fee)))
            && (self
                .private_key
                .as_ref()
                .map_or(true, |key| bot.private_key.as_ref() == Some(key)))
            && (self.contract_address.as_ref().map_or(true, |address| {
                bot.contract_address.as_ref() == Some(address)
            }))
    }

    /// Builder-style setter for `bot_id`
    /// Builder-style setter for `bot_id`
    pub fn bot_id(mut self, bot_id: Option<String>) -> Self {
        self.bot_id = bot_id;
        self
    }

    /// Builder-style setter for `name`
    pub fn name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    /// Builder-style setter for `exchange`
    pub fn exchange(mut self, exchange: Option<String>) -> Self {
        self.exchange = exchange;
        self
    }

    /// Builder-style setter for `api_key`
    pub fn api_key(mut self, api_key: Option<String>) -> Self {
        self.api_key = api_key;
        self
    }

    // /// Builder-style setter for `api_secret`
    // pub fn api_secret(mut self, api_secret: Option<String>) -> Self {
    //     self.api_secret = api_secret;
    //     self
    // }

    /// Builder-style setter for `rest_endpoint`
    pub fn rest_endpoint(mut self, rest_endpoint: Option<String>) -> Self {
        self.rest_endpoint = rest_endpoint;
        self
    }

    /// Builder-style setter for `rpc_endpoint`
    pub fn rpc_endpoint(mut self, rpc_endpoint: Option<String>) -> Self {
        self.rpc_endpoint = rpc_endpoint;
        self
    }

    // /// Builder-style setter for `webhook_secret`
    // pub fn webhook_secret(mut self, webhook_secret: Option<String>) -> Self {
    //     self.webhook_secret = webhook_secret;
    //     self
    // }

    /// Builder-style setter for `trading_fee`
    pub fn trading_fee(mut self, trading_fee: Option<f64>) -> Self {
        self.trading_fee = trading_fee;
        self
    }

    /// Builder-style setter for `private_key`
    pub fn private_key(mut self, private_key: Option<String>) -> Self {
        self.private_key = private_key;
        self
    }

    /// Builder-style setter for `contract_address`
    pub fn contract_address(mut self, contract_address: Option<String>) -> Self {
        self.contract_address = contract_address;
        self
    }

    /// Builder-style setter for `listeners`
    #[allow(dead_code)]
    pub fn listeners(mut self, listeners: HashMap<String, Listener>) -> Self {
        self.listeners = listeners;
        self
    }
}
