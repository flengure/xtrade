pub use crate::bot::cli::Cli;
use crate::bot::cli::Commands;
use crate::bot::rest::RestClient;
use crate::bot::server;
use crate::bot::server::ServerArgs;
use crate::bot::state::AppState;
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};

/// Handle CLI commands and modes
pub async fn run(cli: Cli, app_state: Arc<Mutex<AppState>>) -> Result<()> {
    match cli.mode() {
        "server" => {
            if let Commands::Server {
                port,
                bind,
                state,
                web,
                no_web,
                web_port,
                web_bind,
                web_path,
            } = cli.command
            {
                // Construct the `ServerArgs` struct and pass it to the `server::run` function
                let server_args = ServerArgs {
                    port,
                    bind,
                    state,
                    web,
                    no_web,
                    web_port,
                    web_bind,
                    web_root: web_path,
                };
                server::run(server_args, app_state.clone()).await?;
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Invalid command for server mode.",
                ));
            }
        }
        "offline" => {
            // Pass `cli.command` directly since it's not wrapped in `Option`
            crate::bot::cli::offline::run(cli.command, app_state.clone())
                .await
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        }
        "online" => {
            if let Some(url) = cli.url {
                let rest_client = RestClient::new(&url);
                // Pass `cli.command` directly since it's not wrapped in `Option`
                crate::bot::cli::online::run(cli.command, rest_client)
                    .await
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "URL must be provided for online mode.",
                ));
            }
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Unknown or unsupported mode.",
            ));
        }
    }
    Ok(())
}
