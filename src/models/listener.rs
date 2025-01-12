use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listener {
    pub listener_id: String, // Unique ID for each listener
    pub service: String,     // Service type (e.g., TradingView)
    pub secret: String,      // Security secret for the webhook
    pub msg: Value,          // Change msg to serde_json::Value
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
