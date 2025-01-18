//use crate::models::Listener;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
