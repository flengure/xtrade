use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    pub service: String, // Service type (e.g., TradingView)
    pub secret: String,  // Security secret for the webhook
    pub msg: Value,      // Change msg to serde_json::Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerInsert {
    pub bot_id: String,  // Bot ID for this listener
    pub service: String, // Service type (e.g., TradingView)
    #[serde(skip_serializing)]
    pub secret: Option<String>, // Security secret for the webhook
    pub msg: Option<Value>, // Change msg to serde_json::Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerUpdate {
    pub bot_id: Option<String>,  // Bot ID for this listener
    pub listener_id: String,     // Unique ID for each listener
    pub service: Option<String>, // Service type (e.g., TradingView)
    #[serde(skip_serializing)]
    pub secret: Option<String>, // Security secret for the webhook
    pub msg: Option<Value>,      // Change msg to serde_json::Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerView {
    pub bot_id: String,          // Bot ID for this listener
    pub listener_id: String,     // Unique ID for each listener
    pub service: Option<String>, // Service type (e.g., TradingView)
    #[serde(skip_serializing)]
    pub secret: Option<String>, // Security secret for the webhook
    pub msg: Option<Value>,      // Change msg to serde_json::Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerDelete {
    pub bot_id: Option<String>,      // Bot ID for this listener
    pub listener_id: Option<String>, // Unique ID for each listener
    pub service: Option<String>,     // Service type (e.g., TradingView)
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListenerList {
    #[validate(length(min = 1, message = "Bot ID cannot be empty"))]
    pub bot_id: String, // Bot ID for this listener
    pub listener_id: Option<String>, // Unique ID for each listener
    pub service: Option<String>,     // Service type (e.g., TradingView)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerGet {
    pub bot_id: Option<String>,      // Bot ID for this listener
    pub listener_id: Option<String>, // Unique ID for each listener
}

impl Listener {
    /// Generate the webhook message for this listener
    pub fn generate_message(service: &str, bot_id: &str) -> serde_json::Value {
        match service {
            "TradingView" => serde_json::json!({
                "bot_id": bot_id,
                "ticker": "{{ticker}}",
                "action": "{{strategy.order.action}}",
                "order_size": "100%",
                "position_size": "{{strategy.position_size}}",
                "schema": "2",
                "timestamp": "{{time}}"
            }),
            "Telegram" => serde_json::json!({
                "text": "🚨 *{{ticker}}* is *{{action}}* at `{{close}}`"
            }),
            "Discord" => serde_json::json!({
                "content": "**{{ticker}}** is **{{action}}** at `{{close}}`"
            }),
            "Slack" => serde_json::json!({
                "text": ":rotating_light: *{{ticker}}* is *{{action}}* at `{{close}}`"
            }),
            _ => serde_json::json!({
                "alert": "Alert: {{ticker}} is {{action}}"
            }),
        }
    }
}
