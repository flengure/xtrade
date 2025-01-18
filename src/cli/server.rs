// src/cli/server.rs
use crate::bot::AppState;
use crate::cli::ServerArgs;
use crate::utils::config::AppConfig;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::Mutex;

/// Run the application in server mode
pub async fn server_mode(args: ServerArgs) -> std::io::Result<()> {
    // Load configuration using AppConfig
    let config = AppConfig::load(None).unwrap_or_else(|err| {
        log::warn!(
            "Failed to load configuration file: {}. Using defaults.",
            err
        );
        AppConfig::default()
    });
    log::debug!("Loaded configuration: {:?}", config);

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

    let app_state = web::Data::new(Mutex::new(
        AppState::load(Some(&state_file)).unwrap_or_else(|_| AppState::default()),
    ));

    // Create the API server
    let api_server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(crate::api::init_routes)
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

// #[cfg(test)]
// mod sync_tests {
//     //use super::*;
//     use crate::cli::ServerArgs;
//     use crate::utils::config::AppConfig;
//     use std::fs;
//
//     /// Helper function to create a temporary config file
//     fn create_temporary_config_file(content: &str, file_name: &str) {
//         fs::write(file_name, content).expect("Failed to write temporary config file");
//     }
//
//     /// Helper function to delete a temporary file
//     fn delete_temporary_file(file_name: &str) {
//         let _ = fs::remove_file(file_name);
//     }
//
//     #[test]
//     fn test_server_mode_with_defaults() {
//         let args = ServerArgs {
//             bind: None,
//             port: None,
//             state: None,
//             web: None,
//             no_web: None,
//         };
//
//         let config = AppConfig::default();
//         let bind = args.bind.unwrap_or_else(|| config.server.bind.clone());
//         let port = args.port.unwrap_or(config.server.port);
//         let state_file = args.state.unwrap_or_else(|| config.server.state.clone());
//
//         assert_eq!(bind, "127.0.0.1");
//         assert_eq!(port, 7762);
//         assert_eq!(state_file, "state.json");
//     }
//
//     #[test]
//     fn test_server_mode_with_arguments() {
//         let args = ServerArgs {
//             bind: Some("0.0.0.0".to_string()),
//             port: Some(8000),
//             state: Some("custom_state.json".to_string()),
//         };
//
//         let config = AppConfig::default();
//         let bind = args.bind.unwrap_or_else(|| config.server.bind.clone());
//         let port = args.port.unwrap_or(config.server.port);
//         let state_file = args.state.unwrap_or_else(|| config.server.state.clone());
//
//         assert_eq!(bind, "0.0.0.0");
//         assert_eq!(port, 8000);
//         assert_eq!(state_file, "custom_state.json");
//     }
//
//     #[test]
//     fn test_server_mode_with_config_file() {
//         let config_content = r#"
//         [server]
//         bind = "192.168.1.1"
//         port = 9090
//         state = "config_state.json"
//
//         [web]
//         enabled = true
//         port = 7763
//         bind = "0.0.0.0"
//         path = "src/webui/dist"
//         "#;
//
//         let file_name = "config.toml";
//         create_temporary_config_file(config_content, file_name);
//
//         let config = AppConfig::load(Some(file_name)).unwrap();
//         assert_eq!(config.server.bind, "192.168.1.1");
//         assert_eq!(config.server.port, 9090);
//         assert_eq!(config.server.state, "config_state.json");
//         assert!(config.web.enabled);
//         assert_eq!(config.web.port, 7763);
//         assert_eq!(config.web.bind, "0.0.0.0");
//         assert_eq!(config.web.path, "src/webui/dist");
//
//         delete_temporary_file(file_name);
//     }
// }
//
// #[cfg(test)]
// mod async_tests {
//     use super::*;
//     use actix_web::{test, web, App, HttpResponse};
//     use std::sync::Mutex;
//
//     #[actix_web::test]
//     async fn test_server_mode_bots_endpoint() {
//         // Mock AppState
//         let app_state = web::Data::new(Mutex::new(AppState::default()));
//
//         // Mock API routes
//         fn mock_api_routes(cfg: &mut web::ServiceConfig) {
//             cfg.route(
//                 "/bots",
//                 web::get().to(|| async { HttpResponse::Ok().body("[]") }),
//             );
//         }
//
//         // Initialize test server for the API
//         let api_server = test::init_service(
//             App::new()
//                 .app_data(app_state.clone())
//                 .configure(mock_api_routes),
//         )
//         .await;
//
//         // Test API route: /bots
//         let req = test::TestRequest::get().uri("/bots").to_request();
//         let resp = test::call_service(&api_server, req).await;
//
//         // Assert response status is 200 OK
//         assert!(resp.status().is_success());
//
//         // Assert response body is an empty list
//         let body = test::read_body(resp).await;
//         assert_eq!(body, "[]");
//     }
//
//     #[actix_web::test]
//     async fn test_server_mode_invalid_route() {
//         // Mock AppState
//         let app_state = web::Data::new(Mutex::new(AppState::default()));
//
//         // Initialize test server without specific routes
//         let server = test::init_service(App::new().app_data(app_state.clone())).await;
//
//         // Test an invalid route
//         let req = test::TestRequest::get().uri("/invalid").to_request();
//         let resp = test::call_service(&server, req).await;
//
//         // Assert response status is 404 Not Found
//         assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_FOUND);
//     }
//
//     #[actix_web::test]
//     async fn test_web_ui_server_static_files() {
//         // Mock Web UI static file service
//         let web_ui_path = "src/webui/dist";
//         let web_server = test::init_service(
//             App::new().service(fs::Files::new("/", web_ui_path).index_file("index.html")),
//         )
//         .await;
//
//         // Test Web UI route (e.g., loading index.html)
//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = test::call_service(&web_server, req).await;
//
//         // Assert response status is 200 OK
//         assert!(resp.status().is_success());
//
//         // Assert response body contains the expected content (assuming a known test HTML file)
//         let body = test::read_body(resp).await;
//         assert!(std::str::from_utf8(&body)
//             .unwrap()
//             .contains("<!doctype html>")); // Basic validation for an HTML file
//     }
//
//     #[actix_web::test]
//     async fn test_combined_api_and_web_ui() {
//         // Mock AppState for the API
//         let app_state = web::Data::new(Mutex::new(AppState::default()));
//
//         // Mock API routes
//         fn mock_api_routes(cfg: &mut web::ServiceConfig) {
//             cfg.route(
//                 "/bots",
//                 web::get().to(|| async { HttpResponse::Ok().body("[]") }),
//             );
//         }
//
//         // Define Web UI path
//         let web_ui_path = "src/webui/dist";
//
//         // Initialize combined server
//         let server = test::init_service(
//             App::new()
//                 .app_data(app_state.clone())
//                 .configure(mock_api_routes)
//                 .service(fs::Files::new("/", web_ui_path).index_file("index.html")),
//         )
//         .await;
//
//         // Test API route
//         let api_req = test::TestRequest::get().uri("/bots").to_request();
//         let api_resp = test::call_service(&server, api_req).await;
//         assert!(api_resp.status().is_success());
//         let api_body = test::read_body(api_resp).await;
//         assert_eq!(api_body, "[]");
//
//         // Test Web UI route
//         let web_req = test::TestRequest::get().uri("/").to_request();
//         let web_resp = test::call_service(&server, web_req).await;
//         assert!(web_resp.status().is_success());
//         let web_body = test::read_body(web_resp).await;
//         assert!(std::str::from_utf8(&web_body)
//             .unwrap()
//             .contains("<!doctype html>"));
//     }
// }
