// src/lib.rs

pub mod api;
pub mod app_state;
pub mod cli;
pub mod models;
pub mod services;
pub mod utils; // If you have REST APIs in `api/`

// Optionally re-export common items to simplify imports
pub use app_state::AppState;
pub use cli::Cli;
