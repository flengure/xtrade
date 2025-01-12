// src/api/listeners.rs

use crate::app_state::AppState;
use crate::models::Listener;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[get("/bots/{bot_id}/listeners")]
async fn get_listeners(
    data: web::Data<Mutex<AppState>>,
    bot_id: web::Path<String>,
) -> impl Responder {
    let state = data.lock().unwrap();

    if let Some(bot) = state.bots.get(&bot_id.into_inner()) {
        return HttpResponse::Ok().json(&bot.listeners);
    }

    HttpResponse::NotFound().body("Bot not found")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerInput {
    pub service: String,
    pub secret: String,
    pub msg: Option<String>,
}

#[post("/bots/{bot_id}/listeners")]
async fn add_listener(
    data: web::Data<Mutex<AppState>>,
    path: web::Path<String>,
    listener_input: Result<web::Json<ListenerInput>, actix_web::Error>, // Handle deserialization errors
) -> impl Responder {
    let bot_id = path.into_inner();
    let mut state = data.lock().unwrap();

    let listener_input = match listener_input {
        Ok(input) => input,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid input payload");
        }
    };

    if let Some(bot) = state.bots.get_mut(&bot_id) {
        let msg = if let Some(msg) = &listener_input.msg {
            serde_json::from_str(msg).unwrap_or_else(|_| serde_json::Value::String(msg.clone()))
        } else {
            Listener::generate_message(&listener_input.service, &bot_id)
        };

        let listener = Listener {
            listener_id: uuid::Uuid::new_v4().to_string(),
            service: listener_input.service.clone(),
            secret: listener_input.secret.clone(),
            msg,
        };

        bot.listeners.push(listener);

        state.save_state_to_file("state.json");
        return HttpResponse::Created().body("Listener added successfully");
    }

    HttpResponse::NotFound().body("Bot not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::AppState;
    use crate::models::{Bot, Listener};
    use actix_web::{test, web, App};
    use serde_json::json;
    use std::collections::HashMap;
    use std::str;

    /// Mock an empty `AppState`.
    fn mock_app_state() -> web::Data<Mutex<AppState>> {
        web::Data::new(Mutex::new(AppState {
            bots: HashMap::new(),
        }))
    }

    #[actix_web::test]
    async fn test_get_listeners_no_listeners() {
        let app_state = mock_app_state();

        // Add a bot with no listeners
        let bot_id = "bot123".to_string();
        app_state.lock().unwrap().bots.insert(
            bot_id.clone(),
            Bot {
                bot_id: bot_id.clone(),
                name: "TestBot".to_string(),
                exchange: "Binance".to_string(),
                api_key: None,
                api_secret: None,
                rest_endpoint: None,
                rpc_endpoint: None,
                webhook_secret: None,
                trading_fee: None,
                private_key: None,
                contract_address: None,
                listeners: Vec::new(),
            },
        );

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(get_listeners),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/bots/{}/listeners", bot_id))
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        // Convert Bytes to String and assert the response
        let resp_str = str::from_utf8(&resp).expect("Invalid UTF-8 in response");
        assert_eq!(resp_str, "[]"); // Expecting an empty list of listeners
    }

    #[actix_web::test]
    async fn test_get_listeners_with_listeners() {
        let app_state = mock_app_state();

        // Add a bot with listeners
        let bot_id = "bot123".to_string();
        app_state.lock().unwrap().bots.insert(
            bot_id.clone(),
            Bot {
                bot_id: bot_id.clone(),
                name: "TestBot".to_string(),
                exchange: "Binance".to_string(),
                api_key: None,
                api_secret: None,
                rest_endpoint: None,
                rpc_endpoint: None,
                webhook_secret: None,
                trading_fee: None,
                private_key: None,
                contract_address: None,
                listeners: vec![Listener {
                    listener_id: "listener1".to_string(),
                    service: "TestService".to_string(),
                    secret: "secret123".to_string(),
                    msg: json!({"key": "value"}),
                }],
            },
        );

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(get_listeners),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/bots/{}/listeners", bot_id))
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        let body = String::from_utf8(resp.to_vec()).unwrap();
        assert!(body.contains("TestService"));
    }

    #[actix_web::test]
    async fn test_get_listeners_bot_not_found() {
        let app_state = mock_app_state();
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(get_listeners),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/bots/unknown/listeners")
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        // Convert Bytes to String and assert the response
        let resp_str = str::from_utf8(&resp).expect("Invalid UTF-8 in response");
        assert_eq!(resp_str, "Bot not found");
    }

    #[actix_web::test]
    async fn test_add_listener_success() {
        let app_state = mock_app_state();

        // Add a bot to the app state
        let bot_id = "bot123".to_string();
        app_state.lock().unwrap().bots.insert(
            bot_id.clone(),
            Bot {
                bot_id: bot_id.clone(),
                name: "TestBot".to_string(),
                exchange: "Binance".to_string(),
                api_key: None,
                api_secret: None,
                rest_endpoint: None,
                rpc_endpoint: None,
                webhook_secret: None,
                trading_fee: None,
                private_key: None,
                contract_address: None,
                listeners: Vec::new(),
            },
        );

        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(add_listener)).await;

        let listener_input = ListenerInput {
            service: "TestService".to_string(),
            secret: "secret123".to_string(),
            msg: Some(r#"{"key": "value"}"#.to_string()),
        };

        let req = test::TestRequest::post()
            .uri(&format!("/bots/{}/listeners", bot_id))
            .set_json(&listener_input)
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        // Convert Bytes to String and assert the response
        let resp_str = str::from_utf8(&resp).expect("Invalid UTF-8 in response");
        assert_eq!(resp_str, "Listener added successfully");

        // Verify the listener was added
        let listeners = &app_state.lock().unwrap().bots[&bot_id].listeners;
        assert_eq!(listeners.len(), 1);
        assert_eq!(listeners[0].service, "TestService");
    }

    #[actix_web::test]
    async fn test_add_listener_bot_not_found() {
        let app_state = mock_app_state();
        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(add_listener)).await;

        let listener_input = ListenerInput {
            service: "TestService".to_string(),
            secret: "secret123".to_string(),
            msg: None,
        };

        let req = test::TestRequest::post()
            .uri("/bots/unknown/listeners")
            .set_json(&listener_input)
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        // Convert Bytes to String and assert the response
        let resp_str = str::from_utf8(&resp).expect("Invalid UTF-8 in response");
        assert_eq!(resp_str, "Bot not found");
    }

    #[actix_web::test]
    async fn test_add_listener_invalid_input() {
        let app_state = mock_app_state();

        // Add a bot to the app state
        let bot_id = "bot123".to_string();
        app_state.lock().unwrap().bots.insert(
            bot_id.clone(),
            Bot {
                bot_id: bot_id.clone(),
                name: "TestBot".to_string(),
                exchange: "Binance".to_string(),
                api_key: None,
                api_secret: None,
                rest_endpoint: None,
                rpc_endpoint: None,
                webhook_secret: None,
                trading_fee: None,
                private_key: None,
                contract_address: None,
                listeners: Vec::new(),
            },
        );

        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(add_listener)).await;

        // Invalid JSON payload
        let invalid_listener_input = r#"{"invalid": "data"}"#;

        let req = test::TestRequest::post()
            .uri(&format!("/bots/{}/listeners", bot_id))
            .set_payload(invalid_listener_input) // Send invalid payload
            .insert_header(("Content-Type", "application/json"))
            .to_request();

        let resp = test::call_and_read_body(&app, req).await;

        // Convert Bytes to String and assert the response
        let resp_str = String::from_utf8(resp.to_vec()).expect("Invalid UTF-8 in response");
        assert_eq!(resp_str, "Invalid input payload");
    }
}
