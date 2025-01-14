// src/cli/handlers.rs
use crate::app_state::AppState;
use crate::cli::Commands;
use crate::cli::OfflineArgs;
use crate::models::{BotInsert, BotUpdate, Listener};
use crate::utils::config::AppConfig; // Import the AppConfig
use log::{error, info};
use serde_json::Value;
