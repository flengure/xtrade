use crate::bot::model::Bot;
use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Args, Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BotListArgs {
    // these 2 are here because of clap
    // ignored by other consumers
    #[arg(long)]
    pub page: Option<u32>,
    #[arg(long)]
    pub limit: Option<u32>,
    // regular input fields
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
    #[allow(dead_code)]
    pub fn bot_id(mut self, bot_id: Option<&str>) -> Self {
        self.bot_id = bot_id.map(|x| x.to_string());
        self
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
}
