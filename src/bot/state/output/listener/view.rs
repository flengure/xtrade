//use crate::models::Listener;
pub use crate::bot::model::Listener;
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

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
