use crate::app_config::AppConfig;
use crate::app_state::AppState;
use crate::bot::cli::{Cli, Commands};
use crate::bot::rest::RestClient;
use crate::bot::server;
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};

/// Main function to handle CLI commands and modes
pub async fn run(cli: Cli, app_config: AppConfig, app_state: Arc<Mutex<AppState>>) -> Result<()> {
    match cli.mode() {
        "server" => run_server_mode(cli, app_config, app_state).await,
        "offline" => run_offline_mode(cli, app_state).await,
        "online" => run_online_mode(cli).await,
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "Unknown or unsupported mode.",
        )),
    }
}

/// Handle server mode
async fn run_server_mode(
    cli: Cli,
    app_config: AppConfig,
    app_state: Arc<Mutex<AppState>>,
) -> Result<()> {
    if let Commands::Server(server_args) = cli.command {
        server::run(server_args, app_config, app_state).await?;
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid command for server mode.",
        ))
    }
}

/// Handle offline mode
async fn run_offline_mode(cli: Cli, app_state: Arc<Mutex<AppState>>) -> Result<()> {
    if let Commands::Offline { offline_command } = cli.command {
        crate::bot::cli::offline::run(offline_command, app_state)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid command for offline mode.",
        ))
    }
}

/// Handle online mode
async fn run_online_mode(cli: Cli) -> Result<()> {
    let rest_client = RestClient::new(&cli.url.unwrap());
    crate::bot::cli::online::run(cli.command, rest_client)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))
}
