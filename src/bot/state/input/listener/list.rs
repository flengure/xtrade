//use crate::models::Listener;
use crate::bot::model::Listener;
use clap::Args;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Args, Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerListArgs {
    #[arg(long)]
    pub page: Option<u32>,
    #[arg(long)]
    pub limit: Option<u32>,
    #[arg(long)]
    pub bot_id: String,
    #[arg(long)]
    pub listener_id: Option<String>,
    #[arg(long)]
    pub service: Option<String>,
}

impl ListenerListArgs {
    /// Create a new `ListenerListArgs` with mandatory `bot_id`
    pub fn new(bot_id: String) -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
            bot_id,
            listener_id: None,
            service: None,
        }
    }

    /// Add optional `page` to filter by a specific listener
    pub fn page(mut self, page: Option<u32>) -> Self {
        self.page = page;
        self
    }
    /// Add optional `limit` to filter by a specific listener
    pub fn limit(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }
    /// Add optional `bot_id` to filter by a specific listener
    pub fn bot_id(mut self, bot_id: String) -> Self {
        self.bot_id = bot_id;
        self
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
