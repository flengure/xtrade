// src/main.rs
mod bot;
mod config;
mod errors;
mod state;
mod utils;

use crate::bot::state::AppState;
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use fern::Dispatch;
use log::LevelFilter;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("Failed to initialize logger");
    dotenv().ok();

    // Try to load AppState from a file or fall back to the default state
    let app_state = Arc::new(Mutex::new(
        AppState::load(None::<std::path::PathBuf>)
            .map_err(|err| {
                log::warn!("Failed to load AppState: {}. Using default state.", err);
                err
            })
            .unwrap_or_else(|_| AppState::default()),
    ));

    // Parse CLI arguments and delegate to bot::cli::run
    let _ = bot::cli::run(bot::cli::Cli::parse(), app_state.clone()).await;

    Ok(())
}

/// Configure and set up the logger
fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            let level_color = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".cyan(),
            };

            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_color,
                message
            ));
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
