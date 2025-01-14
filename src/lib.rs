// src/lib.rs

pub mod api;
pub mod app_state;
pub mod cli;
pub mod errors;
pub mod models;
pub mod services;
pub mod utils;

// Optionally re-export common items to simplify imports
pub use app_state::AppState;
pub use cli::Cli;
