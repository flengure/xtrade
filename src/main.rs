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
//use errors::ServerError;
use log::error;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Initialize the logger, mapping fern::InitError into std::io::Error
    setup_logger().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Load AppConfig
    let app_config = match AppConfig::load::<&Path>(None) {
        Ok(config) => config, // Successfully loaded
        Err(e) => {
            error!("Failed to load AppConfig: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    // Proceed with the rest of the application
    // println!("{:?}", &app_config);

    // Try to load AppState from a file or fall back to the default state
    let app_state = Arc::new(Mutex::new(
        AppState::load(app_config).map_err(map_to_io_error)?,
    ));

    // // Proceed with the rest of the application
    println!("{:?}", &app_state);

    // let cli = bot::cli::Cli::parse();
    Ok(())
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
