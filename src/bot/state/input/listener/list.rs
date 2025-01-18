//use crate::models::Listener;
pub use crate::bot::model::Listener;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
