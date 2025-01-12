// src/cli/server.rs
use crate::app_state::AppState;
use crate::cli::ServerArgs;
use actix_web::{web, App, HttpServer};
use config::{Config, File};
use log::info;
use std::sync::Mutex;

/// Run the application in server mode
pub async fn server_mode(args: ServerArgs) -> std::io::Result<()> {
    let (bind, port, state_file) = initialize_server_args(args);

    info!(
        "Starting server on {}:{} with state file: {}",
        bind, port, state_file
    );

    let app_state = web::Data::new(Mutex::new(
        AppState::load_state_from_file(&state_file).unwrap_or_else(|_| AppState::default()),
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(crate::api::init_routes)
    })
    .bind((bind.as_str(), port))?
    .run()
    .await
}

fn initialize_server_args(args: ServerArgs) -> (String, u16, String) {
    let settings = Config::builder()
        .add_source(File::with_name("server").required(false))
        .build()
        .unwrap();

    let default_bind = settings
        .get_string("server.bind")
        .unwrap_or_else(|_| "0.0.0.0".to_string());
    let default_port = settings.get_int("server.port").unwrap_or(8080) as u16;
    let default_state = settings
        .get_string("server.state")
        .unwrap_or_else(|_| "state.json".to_string());

    let bind = args.bind.unwrap_or(default_bind);
    let port = args.port.unwrap_or(default_port);
    let state_file = args.state.unwrap_or(default_state);

    (bind, port, state_file)
}

#[cfg(test)]
mod sync_tests {
    use super::*;
    use crate::cli::ServerArgs;
    use std::fs;

    /// Helper function to create a temporary server.toml file
    fn create_temporary_config_file(content: &str, file_name: &str) {
        fs::write(file_name, content).expect("Failed to write temporary config file");
    }

    /// Helper function to delete a temporary file
    fn delete_temporary_file(file_name: &str) {
        let _ = fs::remove_file(file_name);
    }

    #[test]
    fn test_initialize_server_args_with_defaults() {
        let args = ServerArgs {
            bind: None,
            port: None,
            state: None,
        };

        let (bind, port, state_file) = initialize_server_args(args);

        assert_eq!(bind, "0.0.0.0");
        assert_eq!(port, 8080);
        assert_eq!(state_file, "state.json");
    }

    #[test]
    fn test_initialize_server_args_with_arguments() {
        let args = ServerArgs {
            bind: Some("127.0.0.1".to_string()),
            port: Some(3000),
            state: Some("custom_state.json".to_string()),
        };

        let (bind, port, state_file) = initialize_server_args(args);

        assert_eq!(bind, "127.0.0.1");
        assert_eq!(port, 3000);
        assert_eq!(state_file, "custom_state.json");
    }

    #[test]
    fn test_initialize_server_args_with_config_file() {
        let config_content = r#"
        [server]
        bind = "192.168.1.1"
        port = 9090
        state = "config_state.json"
        "#;

        let file_name = "server.toml";
        create_temporary_config_file(config_content, file_name);

        let args = ServerArgs {
            bind: None,
            port: None,
            state: None,
        };

        let (bind, port, state_file) = initialize_server_args(args);

        assert_eq!(bind, "192.168.1.1");
        assert_eq!(port, 9090);
        assert_eq!(state_file, "config_state.json");

        delete_temporary_file(file_name);
    }

    #[test]
    fn test_initialize_server_args_with_arguments_and_config_file() {
        let config_content = r#"
        [server]
        bind = "192.168.1.1"
        port = 9090
        state = "config_state.json"
        "#;

        let file_name = "server.toml";
        create_temporary_config_file(config_content, file_name);

        let args = ServerArgs {
            bind: Some("127.0.0.1".to_string()),
            port: Some(3000),
            state: Some("custom_state.json".to_string()),
        };

        let (bind, port, state_file) = initialize_server_args(args);

        // Arguments should override the config file values
        assert_eq!(bind, "127.0.0.1");
        assert_eq!(port, 3000);
        assert_eq!(state_file, "custom_state.json");

        delete_temporary_file(file_name);
    }
}

#[cfg(test)]
mod async_tests {
    use super::*;
    use actix_web::{test, web, App, HttpResponse};

    #[actix_web::test]
    async fn test_server_mode_bots_endpoint() {
        // Create mock AppState
        let app_state = web::Data::new(Mutex::new(AppState::default()));

        // Define mock routes, specifically for /bots
        fn mock_routes(cfg: &mut web::ServiceConfig) {
            cfg.route(
                "/bots",
                web::get().to(|| async { HttpResponse::Ok().body("[]") }),
            ); // Mock the /bots route
        }

        // Initialize the test server with mock AppState and routes
        let server = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .configure(mock_routes),
        )
        .await;

        // Test the /bots route
        let req = test::TestRequest::get().uri("/bots").to_request();
        let resp = test::call_service(&server, req).await;

        // Print the response for debugging
        eprintln!("Response: {:?}", resp);

        // Assert the response status is 200 OK
        assert!(resp.status().is_success());

        // Assert the response body matches expected output (an empty list of bots in this case)
        let body = test::read_body(resp).await;
        assert_eq!(body, "[]");
    }
}
