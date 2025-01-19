// src/main.rs
mod bot;
mod errors;
mod state;
mod utils;

use crate::bot::state::AppState;
use crate::errors::map_to_io_error;
use crate::utils::logging::setup_logger;
use clap::Parser;
use dotenv::dotenv;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("Failed to initialize logger");
    dotenv().ok();

    // Try to load AppState from a file or fall back to the default state
    let app_state = Arc::new(Mutex::new(
        AppState::load(None::<std::path::PathBuf>).map_err(map_to_io_error)?,
    ));

    // Parse CLI arguments and delegate to bot::cli::run
    bot::cli::run(bot::cli::Cli::parse(), app_state.clone()).await
}
