//use crate::models::Listener;
pub use crate::bot::model::Listener;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
