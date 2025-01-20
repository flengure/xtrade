//use crate::models::Listener;
use clap::Args;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct ListenerInsertArgs {
    #[arg(long)]
    pub bot_id: String,
    #[arg(long)]
    pub listener_id: Option<String>,
    #[arg(long)]
    #[validate(length(min = 1, message = "Service cannot be empty"))]
    pub service: String,
    #[arg(long)]
    pub secret: Option<String>,
    #[arg(long)]
    pub msg: Option<String>,
}

impl ListenerInsertArgs {
    /// Creates a new `ListenerInsertArgs` instance with required fields.
    /// Generates a `listener_id` if none is provided.
    #[allow(dead_code)]
    pub fn new(bot_id: &str, service: &str) -> Self {
        Self {
            bot_id: bot_id.to_string(),
            listener_id: Some(Uuid::new_v4().to_string()),
            service: service.to_string(),
            secret: None,
            msg: None,
        }
    }

    /// Fluent builder-style method for `listener_id`.
    #[allow(dead_code)]
    pub fn bot_id(mut self, bot_id: String) -> Self {
        self.bot_id = bot_id;
        self
    }

    #[allow(dead_code)]
    pub fn listener_id(mut self, listener_id: Option<String>) -> Self {
        self.listener_id = listener_id;
        self
    }

    /// Fluent builder-style method for `service`.
    #[allow(dead_code)]
    pub fn service(mut self, service: String) -> Self {
        self.service = service;
        self
    }

    /// Fluent builder-style method for `secret`.
    #[allow(dead_code)]
    pub fn secret(mut self, secret: Option<String>) -> Self {
        self.secret = secret;
        self
    }

    /// Fluent builder-style method for `msg`.
    #[allow(dead_code)]
    pub fn msg(mut self, msg: Option<String>) -> Self {
        self.msg = msg;
        self
    }
}
