use crate::app_config::AppConfig;
use crate::app_state::AppState;
//use crate::bot::state::ServerStartupArgs;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use clap::Args;
use log::info;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Args, Clone, Debug)]
pub struct ServerStartupArgs {
    #[arg(long)]
    pub api_port: Option<u16>,
    #[arg(long)]
    pub api_bind_address: Option<String>,
    #[arg(long)]
    pub state_file: Option<PathBuf>,
    #[arg(long)]
    pub webhook_port: Option<u16>,
    #[arg(long)]
    pub webhook_bind_address: Option<String>,
    /// Enable the Web UI (default)
    #[arg(long, conflicts_with = "web_client_disable")]
    pub web_client_enable: bool,
    /// Disable the Web UI
    #[arg(long, conflicts_with = "web_client")]
    pub web_client_disable: bool,
    #[arg(long)]
    pub web_client_port: Option<u16>,
    #[arg(long)]
    pub web_client_bind_address: Option<String>,
    #[arg(long)]
    pub web_client_static_files: Option<PathBuf>,
}

/// Run the application in server mode
pub async fn run(
    args: ServerStartupArgs,
    app_config: AppConfig,
    app_state: Arc<Mutex<AppState>>,
) -> std::io::Result<()> {
    // Acquire the lock on the AppState
    let app_state_guard = app_state.lock().map_err(|_| {
        log::error!("Failed to acquire lock on AppState.");
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to acquire lock on AppState.",
        )
    })?;

    // Clone the configuration from the locked AppState
    //let app_config = app_state_guard.config.clone();
    drop(app_state_guard); // Explicitly drop the lock to avoid deadlocks later

    // Extract server and web configuration with overrides
    let api_server_bind_address = args
        .api_bind_address
        .unwrap_or_else(|| app_config.api_server.bind_address.clone());
    let api_server_port = args.api_port.unwrap_or(app_config.api_server.port);
    let api_server_state_file = args
        .state_file
        .unwrap_or_else(|| app_config.api_server.state_file.clone());
    let webhook_server_bind_address = args
        .webhook_bind_address
        .unwrap_or_else(|| app_config.webhook_server.bind_address.clone());
    let webhook_server_port = args.webhook_port.unwrap_or(app_config.webhook_server.port);
    let web_client_enable =
        args.web_client_enable || (!args.web_client_disable && app_config.web_client.enable);
    let web_client_bind_address = args
        .web_client_bind_address
        .unwrap_or_else(|| app_config.web_client.bind_address.clone());
    let web_client_port = args.web_client_port.unwrap_or(app_config.web_client.port);
    let web_client_static_files = args
        .web_client_static_files
        .unwrap_or_else(|| app_config.web_client.static_files.clone());

    info!(
        "Starting API server on {}:{} with state file: {}",
        api_server_bind_address,
        api_server_port,
        api_server_state_file.display()
    );

    // Start the API server
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Share the same AppState
            .configure(crate::bot::api::endpoints::configure) // Add routes
    })
    .bind((api_server_bind_address.as_str(), api_server_port))?
    .run();

    // Conditionally start the Web UI server
    if web_client_enable {
        info!(
            "Starting Web UI server on {}:{} serving files from: {}",
            web_client_bind_address,
            web_client_port,
            web_client_static_files.display()
        );

        let web_server = HttpServer::new(move || {
            App::new().service(
                fs::Files::new("/", web_client_static_files.clone()).index_file("index.html"),
            )
            // Serve static files
        })
        .bind((web_client_bind_address.as_str(), web_client_port))?
        .run();

        // Run both servers concurrently
        tokio::select! {
            _ = api_server => {
                info!("API server has stopped.");
            }
            _ = web_server => {
                info!("Web UI server has stopped.");
            }
        }
    } else {
        // Run only the API server
        api_server.await?;
    }

    Ok(())
}
