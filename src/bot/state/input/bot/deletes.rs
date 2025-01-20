//use crate::models::Listener;
use crate::bot::model::Bot;
use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotsDeleteArgs {
    #[arg(long)]
    pub bot_id: Option<String>,
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub exchange: Option<String>,
    #[arg(long)]
    pub api_key: Option<String>,
    #[arg(long)]
    pub rest_endpoint: Option<String>,
    #[arg(long)]
    pub rpc_endpoint: Option<String>,
    #[arg(long)]
    pub trading_fee: Option<f64>,
    #[arg(long)]
    pub private_key: Option<String>,
    #[arg(long)]
    pub contract_address: Option<String>,
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
    pub fn page(mut self, page: Option<u32>) -> Self {
        self.page = page;
        self
    }
    pub fn limit(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }
    pub fn bot_id(mut self, bot_id: Option<String>) -> Self {
        self.bot_id = bot_id;
        self
    }
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
    pub fn rest_endpoint(mut self, rest_endpoint: Option<String>) -> Self {
        self.rest_endpoint = rest_endpoint;
        self
    }
    pub fn rpc_endpoint(mut self, rpc_endpoint: Option<String>) -> Self {
        self.rpc_endpoint = rpc_endpoint;
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
}
