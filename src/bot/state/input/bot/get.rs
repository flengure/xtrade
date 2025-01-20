//use crate::models::Listener;
use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotGetArgs {
    #[arg(long)]
    #[validate(length(min = 1, message = "ID cannot be empty"))]
    pub bot_id: String,
}
impl BotGetArgs {
    /// Create a new instance of `BotGetArgs`
    pub fn new(bot_id: &str) -> Self {
        Self {
            bot_id: bot_id.to_string(),
        }
    }
}
