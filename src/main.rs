mod api;
mod app_state;
mod cli;
mod models;
mod services;
mod utils;

use chrono::Local;
use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use fern::Dispatch;
use log::LevelFilter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("Failed to initialize logger");
    dotenv().ok();

    // Parse CLI arguments
    let cli = cli::Cli::parse();

    // Match the mode
    match cli.mode {
        Some(cli::Mode::Server(server_args)) => {
            println!(
                "Bind: {:?}, Port: {:?}",
                &server_args.bind, server_args.port
            );
            cli::server_mode(server_args).await?;
        }
        Some(cli::Mode::Offline(offline_args)) => {
            cli::offline_mode(offline_args);
        }
        Some(cli::Mode::Online(online_args)) => {
            cli::online_mode(online_args);
        }
        None => {
            // Default to online mode
            cli::online_mode(cli.online);
        }
    }

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
