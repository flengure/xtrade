//use crate::models::Listener;
pub use crate::bot::model::Listener;
use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct ListenerUpdateArgs {
    #[arg(long)]
    pub bot_id: String,
    #[arg(long)]
    #[validate(length(min = 1, message = "Listener ID cannot be empty"))]
    pub listener_id: String, // Required
    #[arg(long)]
    pub service: Option<String>, // Optional
    #[arg(long)]
    pub secret: Option<String>, // Optional
    #[arg(long)]
    pub msg: Option<String>, // Optional
}
impl ListenerUpdateArgs {
    /// Create a new `ListenerUpdateArgs` with mandatory `listener_id`
    pub fn new(bot_id: &str, listener_id: &str) -> Self {
        Self {
            bot_id: bot_id.to_string(),
            listener_id: listener_id.to_string(),
            service: None,
            secret: None,
            msg: None,
        }
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
