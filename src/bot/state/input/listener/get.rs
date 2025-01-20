//use crate::models::Listener;
pub use crate::bot::model::{Bot, Listener};
use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct ListenerGetArgs {
    #[arg(long)]
    #[validate(length(min = 1, message = "ID cannot be empty"))]
    pub bot_id: String,
    #[arg(long)]
    pub listener_id: String,
}
impl ListenerGetArgs {
    /// Create a new instance of `ListenerGetArgs`
    pub fn new(bot_id: &str, listener_id: &str) -> Self {
        Self {
            bot_id: bot_id.to_string(),
            listener_id: listener_id.to_string(),
        }
    }
}
