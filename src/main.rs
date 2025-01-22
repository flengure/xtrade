// src/main.rs
mod app_config;
mod app_state;
mod bot;
mod errors;
mod utils;

use crate::app_config::AppConfig;
use crate::app_state::AppState;
use crate::errors::map_to_io_error;
use crate::utils::logging::setup_logger;
use clap::Parser;
use dotenv::dotenv;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Initialize the logger, mapping fern::InitError into std::io::Error
    setup_logger().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Load AppConfig
    let app_config = AppConfig::load::<&Path>(None).map_err(map_to_io_error)?;

    // Initialize the application state
    let app_state = Arc::new(Mutex::new(
        AppState::load(app_config.clone()).map_err(map_to_io_error)?,
    ));
    let cli = bot::cli::Cli::parse();
    cli.run(app_config, app_state.clone()).await
}
