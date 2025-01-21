// src/main.rs
mod bot;
mod errors;
mod state;
mod utils;

// use crate::bot::state::AppState;
// use crate::errors::map_to_io_error;
// use crate::utils::logging::setup_logger;
// use clap::Parser;
// use dotenv::dotenv;
// use std::sync::{Arc, Mutex};
use crate::state::AppConfig;
use crate::utils::logging::setup_logger;
use errors::ServerError;
use log::{error, info};
use std::path::Path;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

fn main() -> std::io::Result<()> {
    // Initialize the logger, mapping fern::InitError into std::io::Error
    setup_logger().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Load AppConfig
    let app_config = AppConfig::load::<&Path>(None).map_err(|e| {
        error!("Failed to load AppConfig: {}", e);
        ServerError::ConfigError(e)
    });
    // Proceed with the rest of the application
    println!("{:?}", &app_config);

    Ok(())
    //let cli = bot::cli::Cli::parse();
    //let log_level = cli.log_level();
    //setup_logger(log_level).expect("Failed to initialize logger");
    //dotenv().ok();

    // Try to load AppState from a file or fall back to the default state
    // let app_state = Arc::new(Mutex::new(
    //     AppState::load(None::<std::path::PathBuf>).map_err(map_to_io_error)?,
    // ));

    //println!("{:?}", &app_state.config);
    //Ok(())

    // Parse CLI arguments and delegate to bot::cli::run
    // bot::cli::run(bot::cli::Cli::parse(), app_state.clone()).await
}
