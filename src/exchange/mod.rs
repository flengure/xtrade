// src/exchange/mod.rs

pub mod binance;
// pub mod bybit;
// pub mod dydx;
// pub mod hyperliquid;
// pub mod raydium_solana;
// pub mod uniswap_base;

use crate::bot::Bot;
use async_trait::async_trait;
use reqwest::Client;
use std::error::Error;

/// Defines a trait that each exchange must implement.
#[async_trait]
pub trait Exchange {
    /// Executes a trade based on the provided parameters.
    async fn execute_trade(
        &self,
        action: &str,
        symbol_or_contract: &str, // Symbol for CEX or Contract Address for DEX
        price: f64,               // Relevant for limit orders; ignored for market orders on DEXes
        slippage: f64,            // Relevant for DEXes
        bot: &Bot,
        client: &Client,
    ) -> Result<(), Box<dyn Error>>;
}

/// Factory function to create an instance of the appropriate Exchange implementation.
pub fn get_exchange(
    exchange_name: &str,
    _private_key: Option<&str>,
) -> Option<Box<dyn Exchange + Send + Sync>> {
    match exchange_name.to_lowercase().as_str() {
        "binance" => Some(Box::new(binance::BinanceExchange::new())),
        // "bybit" => Some(Box::new(bybit::BybitExchange::new())),
        // "dydx" => Some(Box::new(dydx::DydxExchange::new())),
        // "hyperliquid" => Some(Box::new(hyperliquid::HyperliquidExchange::new())),
        // "uniswap" => Some(Box::new(uniswap_base::UniswapExchange::new(...))),
        // "raydium" => Some(Box::new(raydium_solana::RaydiumExchange::new(...))),
        _ => None,
    }
}
