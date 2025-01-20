use crate::bot::state::AppState;
use crate::bot::state::ServerStartupArgs;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::{Arc, Mutex};

/// Initialize all routes for the API
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Configure bot-related routes
    crate::bot::api::endpoints::configure(cfg);
}

/// Run the application in server mode
pub async fn run(args: ServerStartupArgs, app_state: Arc<Mutex<AppState>>) -> std::io::Result<()> {
    // Acquire the lock on the AppState
    let app_state_guard = app_state.lock().map_err(|_| {
        log::error!("Failed to acquire lock on AppState.");
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to acquire lock on AppState.",
        )
    })?;

    // Clone the configuration from the locked AppState
    let config = app_state_guard.config.clone();
    drop(app_state_guard); // Explicitly drop the lock to avoid deadlocks later

    // Extract server and web configuration with overrides
    let bind = args.bind.unwrap_or_else(|| config.api.bind.clone());
    let port = args.port.unwrap_or(config.api.port);
    let state_file = args.state.unwrap_or_else(|| config.api.state.clone());
    let web_enabled = args.web || (!args.no_web && config.client.enabled);
    let web_bind = args.web_bind.unwrap_or_else(|| config.client.bind.clone());
    let web_port = args.web_port.unwrap_or(config.client.port);
    let web_root = args.web_path.unwrap_or_else(|| config.client.path.clone());

    info!(
        "Starting API server on {}:{} with state file: {}",
        bind, port, state_file
    );

    // Start the API server
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Share the same AppState
            .configure(init_routes) // Add routes
    })
    .bind((bind.as_str(), port))?
    .run();

    // Conditionally start the Web UI server
    if web_enabled {
        info!(
            "Starting Web UI server on {}:{} serving files from: {}",
            web_bind, web_port, web_root
        );

        let web_server = HttpServer::new(move || {
            App::new().service(fs::Files::new("/", web_root.clone()).index_file("index.html"))
            // Serve static files
        })
        .bind((web_bind.as_str(), web_port))?
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
