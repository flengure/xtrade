// src/alert/mod.rs

pub mod tradingview;
// Future modules like telegram can be added here

use crate::alert::tradingview::TradingViewAlert;
// use crate::alert::telegram::TelegramAlert; // Uncomment when added

/// Enum representing different alert sources.
pub enum Alert {
    TradingView(TradingViewAlert),
    // Telegram(TelegramAlert), // Uncomment when added
}

impl Alert {
    /// Parses incoming JSON payload into an Alert enum based on the source.
    pub fn parse(source: &str, json_payload: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match source {
            "tradingview" => {
                let alert: TradingViewAlert = serde_json::from_str(json_payload)?;
                Ok(Alert::TradingView(alert))
            }
            // "telegram" => {
            //     let alert: TelegramAlert = serde_json::from_str(json_payload)?;
            //     Ok(Alert::Telegram(alert))
            // },
            _ => Err("Unsupported alert source".into()),
        }
    }
}
