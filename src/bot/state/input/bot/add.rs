//use crate::models::Listener;
use crate::bot::model::Bot;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotInsertArgs {
    /// Optional unique identifier for the bot. If not provided, a new UUID will be generated.
    #[arg(long)]
    pub bot_id: Option<String>,
    #[arg(long)]
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[arg(long)]
    #[validate(length(min = 1, message = "Exchange cannot be empty"))]
    pub exchange: String,
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
}

#[allow(dead_code)]
impl BotInsertArgs {
    /// Create a new `BotInsertArgs` instance with required fields
    pub fn new(name: String, exchange: String) -> Self {
        //let listeners: HashMap<String, Listener> = HashMap<String, Listener>::new();
        Self {
            bot_id: Some(uuid::Uuid::new_v4().to_string()),
            name,
            exchange,
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: None,
            private_key: None,
            contract_address: None,
        }
    }

    pub fn bot_id(mut self, bot_id: Option<String>) -> Self {
        self.bot_id = bot_id;
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
}

impl From<BotInsertArgs> for Bot {
    fn from(args: BotInsertArgs) -> Self {
        Bot {
            bot_id: args
                .bot_id
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
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
            listeners: HashMap::new(), // Initialize with no listeners
        }
    }
}
