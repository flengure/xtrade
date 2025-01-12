// src/alert/tradingview.rs

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TradingViewAlert {
    pub bot_id: String,
    pub ticker: String,
    pub action: String,
    pub order_size: String,
    pub position_size: String,
    pub schema: String,
    pub timestamp: String,
    // Future fields like order_type, slippage can be added if needed
}

impl TradingViewAlert {
    /// Validates the alert data.
    pub fn validate(&self) -> Result<(), String> {
        if self.bot_id.is_empty() {
            return Err("bot_id is missing".to_string());
        }
        if self.ticker.is_empty() {
            return Err("ticker is missing".to_string());
        }
        if !["buy", "sell"].contains(&self.action.to_lowercase().as_str()) {
            return Err("action must be 'buy' or 'sell'".to_string());
        }
        // Add more validations as needed
        Ok(())
    }
}
