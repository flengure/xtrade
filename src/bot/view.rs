//use crate::models::Listener;
pub use crate::bot::model::{Bot, Listener};
use prettytable::{format, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::vec::IntoIter;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotView {
    pub bot_id: String,
    pub name: String,
    pub exchange: String,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    #[serde(skip_serializing)]
    pub api_secret: Option<String>,
    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,
    #[serde(skip_serializing)]
    pub webhook_secret: Option<String>,
    pub trading_fee: Option<f64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotListView(pub Vec<BotView>);

impl fmt::Display for BotListView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        // Set table format (optional)
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        // Add header row
        table.add_row(Row::new(vec![
            Cell::new("Bot ID"),
            Cell::new("Name"),
            Cell::new("Exchange"),
            Cell::new("Trading Fee"),
        ]));

        // Add rows for each bot
        for bot in &self.0 {
            table.add_row(Row::new(vec![
                Cell::new(&bot.bot_id),
                Cell::new(&bot.name),
                Cell::new(&bot.exchange),
                Cell::new(
                    &bot.trading_fee
                        .map_or("N/A".to_string(), |fee| format!("{:.4}", fee)),
                ),
            ]));
        }

        // Write the table to the formatter
        write!(f, "{}", table)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotInsertArgs {
    /// Optional unique identifier for the bot. If not provided, a new UUID will be generated.
    pub bot_id: Option<String>,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Exchange cannot be empty"))]
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
            listeners: HashMap::new(),
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
    pub fn listeners(mut self, listeners: HashMap<String, Listener>) -> Self {
        self.listeners = listeners;
        self
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerView {
    pub bot_id: String,
    pub listener_id: String,
    pub service: Option<String>,
    #[serde(skip_serializing)]
    pub secret: Option<String>,
    pub msg: Option<String>,
}

impl fmt::Display for ListenerView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Listener ID: {}\nService: {}\nBot ID: {}\nMessage: {}",
            self.listener_id,
            self.service.clone().unwrap_or_else(|| "N/A".to_string()),
            self.bot_id,
            self.msg.clone().unwrap_or_else(|| "N/A".to_string()),
        )
    }
}

impl<'a, B, L> From<(B, L, &'a Listener)> for ListenerView
where
    B: AsRef<str>,
    L: AsRef<str>,
{
    fn from((bot_id, listener_id, listener): (B, L, &'a Listener)) -> Self {
        ListenerView {
            bot_id: bot_id.as_ref().to_string(),
            listener_id: listener_id.as_ref().to_string(),
            service: Some(listener.service.clone()),
            secret: Some(listener.secret.clone()),
            msg: Some(listener.msg.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerListView(pub Vec<ListenerView>);

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerInsertArgs {
    pub listener_id: Option<String>,
    #[validate(length(min = 1, message = "Service cannot be empty"))]
    pub service: String,
    pub secret: Option<String>,
    pub msg: Option<String>,
}

impl ListenerInsertArgs {
    /// Creates a new `ListenerInsertArgs` instance with required fields.
    /// Generates a `listener_id` if none is provided.
    pub fn new(service: String, secret: Option<String>, msg: Option<String>) -> Self {
        Self {
            listener_id: Some(Uuid::new_v4().to_string()),
            service,
            secret,
            msg,
        }
    }

    /// Fluent builder-style method for `listener_id`.
    pub fn listener_id(mut self, listener_id: Option<String>) -> Self {
        self.listener_id = listener_id;
        self
    }

    /// Fluent builder-style method for `service`.
    pub fn service(mut self, service: String) -> Self {
        self.service = service;
        self
    }

    /// Fluent builder-style method for `secret`.
    pub fn secret(mut self, secret: Option<String>) -> Self {
        self.secret = secret;
        self
    }

    /// Fluent builder-style method for `msg`.
    pub fn msg(mut self, msg: Option<String>) -> Self {
        self.msg = msg;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerListArgs {
    pub listener_id: Option<String>,
    pub service: Option<String>,
}

impl ListenerListArgs {
    /// Create a new `ListenerListArgs` with mandatory `bot_id`
    pub fn new() -> Self {
        Self {
            listener_id: None,
            service: None,
        }
    }

    /// Add optional `listener_id` to filter by a specific listener
    pub fn listener_id(mut self, listener_id: Option<String>) -> Self {
        self.listener_id = listener_id;
        self
    }

    /// Add optional `service` to filter by a specific service
    pub fn service(mut self, service: Option<String>) -> Self {
        self.service = service;
        self
    }

    /// Determines whether a given listener matches the filter criteria
    pub fn matches(&self, listener_id: &str, listener: &Listener) -> bool {
        // Check if `listener_id` matches, if provided
        self.listener_id
            .as_ref()
            .map_or(true, |id| id == listener_id)
            &&
        // Check if `service` matches, if provided
        self.service
            .as_ref()
            .map_or(true, |service| service == &listener.service)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerGetArgs {
    pub listener_id: Option<String>,
    pub service: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerUpdateArgs {
    #[validate(length(min = 1, message = "Listener ID cannot be empty"))]
    pub listener_id: String, // Required
    pub service: Option<String>, // Optional
    pub secret: Option<String>,  // Optional
    pub msg: Option<String>,     // Optional
}
impl ListenerUpdateArgs {
    /// Create a new `ListenerUpdateArgs` with mandatory `listener_id`
    pub fn new(listener_id: String) -> Self {
        Self {
            listener_id,
            service: None,
            secret: None,
            msg: None,
        }
    }

    /// Builder-style setter for `service`
    pub fn service(mut self, service: Option<String>) -> Self {
        self.service = service;
        self
    }

    /// Builder-style setter for `secret`
    pub fn secret(mut self, secret: Option<String>) -> Self {
        self.secret = secret;
        self
    }

    /// Builder-style setter for `msg`
    pub fn msg(mut self, msg: Option<String>) -> Self {
        self.msg = msg;
        self
    }

    /// Applies the update arguments to a given `Listener`.
    pub fn apply(&self, listener: &mut Listener) {
        if let Some(service) = &self.service {
            listener.service = service.clone();
        }
        if let Some(secret) = &self.secret {
            listener.secret = secret.clone();
        }
        if let Some(msg) = &self.msg {
            listener.msg = msg.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerDeleteArgs {
    pub listener_id: Option<String>, // Unique ID for each listener
    pub service: Option<String>,     // Service type (e.g., TradingView)
}

impl From<ListenerGetArgs> for ListenerListArgs {
    fn from(args: ListenerGetArgs) -> Self {
        ListenerListArgs {
            listener_id: args.listener_id,
            service: args.service,
        }
    }
}

impl IntoIterator for ListenerListView {
    type Item = ListenerView;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ListenerListView {
    type Item = &'a ListenerView;
    type IntoIter = std::slice::Iter<'a, ListenerView>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ListenerListView {
    type Item = &'a mut ListenerView;
    type IntoIter = std::slice::IterMut<'a, ListenerView>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// Implement Display for ListenerListView
impl fmt::Display for ListenerListView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        // Set the format to remove borders and gridlines
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        // Add the header row
        table.add_row(Row::new(vec![
            Cell::new("Bot ID"),
            Cell::new("Listener ID"),
            Cell::new("Service"),
            Cell::new("Message Preview"),
        ]));

        // Add a separator row (dashes under headers)
        table.add_row(Row::new(vec![
            Cell::new("------"),
            Cell::new("-----------"),
            Cell::new("-------"),
            Cell::new("---------------"),
        ]));

        // Add rows for each listener
        for listener in &self.0 {
            table.add_row(Row::new(vec![
                Cell::new(&listener.bot_id),
                Cell::new(&listener.listener_id),
                Cell::new(
                    &listener
                        .service
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string()),
                ),
                Cell::new(
                    &listener
                        .msg
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string())
                        .chars()
                        .take(10)
                        .collect::<String>(),
                ),
            ]));
        }

        // Write the table to the formatter
        write!(f, "{}", table)
    }
}

impl Listener {
    /// Generate the webhook message for this listener
    pub fn generate_message(service: &str, bot_id: &str) -> serde_json::Value {
        match service {
            "TradingView" => serde_json::json!({
                "bot_id": bot_id,
                "ticker": "{{ticker}}",
                "action": "{{strategy.order.action}}",
                "order_size": "100%",
                "position_size": "{{strategy.position_size}}",
                "schema": "2",
                "timestamp": "{{time}}"
            }),
            "Telegram" => serde_json::json!({
                "text": "🚨 *{{ticker}}* is *{{action}}* at `{{close}}`"
            }),
            "Discord" => serde_json::json!({
                "content": "**{{ticker}}** is **{{action}}** at `{{close}}`"
            }),
            "Slack" => serde_json::json!({
                "text": ":rotating_light: *{{ticker}}* is *{{action}}* at `{{close}}`"
            }),
            _ => serde_json::json!({
                "alert": "Alert: {{ticker}} is {{action}}"
            }),
        }
    }
}
