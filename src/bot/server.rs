use crate::bot::state::AppState;
use crate::config::AppConfig;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use log::{debug, info, warn};
use std::sync::{Arc, Mutex};

pub struct ServerArgs {
    /// Specify the port for the server
    pub port: Option<u16>, // Default value and validated for range

    /// Specify the bind address for the server
    pub bind: Option<String>, // Default value and validated for correctness

    /// Specify the state file for the server
    pub state: Option<String>, // Default value

    /// Enable the Web UI (enabled by default)
    pub web: bool,

    /// Disable the Web UI
    pub no_web: bool,

    /// Specify the port for the web client
    pub web_port: Option<u16>, // Default value and validated for range

    /// Specify the bind address for the web client
    pub web_bind: Option<String>, // Default value and validated for correctness

    /// Specify the root for the web client
    pub web_root: Option<String>, // Default value
}

/// Initialize all routes for the API
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Configure bot-related routes
    crate::bot::api::bots::configure(cfg);

    // Configure listener-related routes
    crate::bot::api::listeners::configure(cfg);
}

/// Run the application in server mode
pub async fn run(args: ServerArgs) -> std::io::Result<()> {
    // Load configuration using AppConfig
    let config = AppConfig::load(None).unwrap_or_else(|err| {
        warn!(
            "Failed to load configuration file: {}. Using defaults.",
            err
        );
        AppConfig::default()
    });
    debug!("Loaded configuration: {:?}", config);

    let server_config = &config.server;
    let web_config = &config.web;

    let bind = args.bind.unwrap_or_else(|| server_config.bind.clone());
    let port = args.port.unwrap_or(server_config.port);
    let state_file = args.state.unwrap_or_else(|| server_config.state.clone());
    let web_enabled = if args.web {
        true
    } else if args.no_web {
        false
    } else {
        web_config.enabled
    };
    let web_bind = args.web_bind.unwrap_or_else(|| web_config.bind.clone());
    let web_port = args.web_port.unwrap_or(web_config.port);
    let web_root = args.web_root.unwrap_or_else(|| web_config.path.clone());
    info!(
        "Starting API server on {}:{} with state file: {}",
        bind, port, state_file
    );

    let app_state = web::Data::new(Arc::new(Mutex::new(
        AppState::load(Some(&state_file)).unwrap_or_else(|_| AppState::default()),
    )));

    // Create the API server
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(init_routes)
    })
    .bind((bind.as_str(), port))?
    .run();

    // If Web UI is enabled, create the Web UI server
    if web_enabled {
        info!(
            "Starting Web UI server on {}:{} serving files from: {}",
            web_bind, web_port, web_root
        );

        let web_server = HttpServer::new(move || {
            App::new().service(fs::Files::new("/", web_root.clone()).index_file("index.html"))
        })
        .bind((web_bind.as_str(), web_port))?
        .run();

        // Use tokio::select! to run both servers concurrently
        tokio::select! {
            _ = api_server => {
                info!("API server has stopped.");
            }
            _ = web_server => {
                info!("Web UI server has stopped.");
            }
        }
    } else {
        // Only run the API server
        api_server.await?;
    }
    Ok(())
}
