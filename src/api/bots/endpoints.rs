// src/api/bots.rs

use crate::app_state::AppState;
use crate::models::Bot;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[get("/bots")]
async fn get_bots(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let state = data.lock().unwrap();
    let bots: Vec<&Bot> = state.bots.values().collect();
    HttpResponse::Ok().json(bots)
}

#[derive(Deserialize, Serialize)]
pub struct BotInput {
    pub name: String,
    pub exchange: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub rest_endpoint: Option<String>,
    pub rpc_endpoint: Option<String>,
    pub webhook_secret: Option<String>,
    pub trading_fee: Option<f64>,
    pub private_key: Option<String>,
    pub contract_address: Option<String>,
}

#[post("/bots")]
async fn add_bot(
    data: web::Data<Mutex<AppState>>,
    bot_input: web::Json<BotInput>,
) -> impl Responder {
    let mut state = data.lock().unwrap();
    let bot_id = Uuid::new_v4().to_string();

    let bot = Bot {
        bot_id: bot_id.clone(),
        name: bot_input.name.clone(),
        exchange: bot_input.exchange.clone(),
        api_key: bot_input.api_key.clone(),
        api_secret: bot_input.api_secret.clone(),
        rest_endpoint: bot_input.rest_endpoint.clone(),
        rpc_endpoint: bot_input.rpc_endpoint.clone(),
        webhook_secret: bot_input.webhook_secret.clone(),
        trading_fee: bot_input.trading_fee.clone(),
        private_key: bot_input.private_key.clone(),
        contract_address: bot_input.contract_address.clone(),
        listeners: Vec::new(), // Initialize with no listeners
    };

    state.bots.insert(bot_id.clone(), bot);
    state.save_state_to_file("state.json");

    HttpResponse::Created().body(format!("Bot added successfully with ID: {}", bot_id))
}

#[get("/bots/{bot_id}")]
async fn get_bot_by_id(
    data: web::Data<Mutex<AppState>>,
    bot_id: web::Path<String>,
) -> impl Responder {
    let state = data.lock().unwrap();
    let bot_id = bot_id.into_inner();

    if let Some(bot) = state.bots.get(&bot_id) {
        HttpResponse::Ok().json(bot)
    } else {
        HttpResponse::NotFound().body("Bot not found")
    }
}

#[put("/bots/{bot_id}")]
async fn update_bot(
    data: web::Data<Mutex<AppState>>,
    bot_id: web::Path<String>,
    bot_input: web::Json<BotInput>,
) -> impl Responder {
    let mut state = data.lock().unwrap();

    if let Some(bot) = state.bots.get_mut(&bot_id.into_inner()) {
        bot.name = bot_input.name.clone();
        bot.exchange = bot_input.exchange.clone();
        bot.api_key = bot_input.api_key.clone();
        bot.api_secret = bot_input.api_secret.clone();
        bot.rest_endpoint = bot_input.rest_endpoint.clone();
        bot.rpc_endpoint = bot_input.rpc_endpoint.clone();
        bot.webhook_secret = bot_input.webhook_secret.clone();
        bot.trading_fee = bot_input.trading_fee.clone();
        bot.private_key = bot_input.private_key.clone();
        bot.contract_address = bot_input.contract_address.clone();
        state.save_state_to_file("state.json");
        return HttpResponse::Ok().body("Bot updated successfully");
    }

    HttpResponse::NotFound().body("Bot not found")
}

#[delete("/bots/{bot_id}")]
async fn delete_bot(data: web::Data<Mutex<AppState>>, bot_id: web::Path<String>) -> impl Responder {
    let mut state = data.lock().unwrap();

    if state.bots.remove(&bot_id.into_inner()).is_some() {
        state.save_state_to_file("state.json");
        return HttpResponse::Ok().body("Bot deleted successfully");
    }

    HttpResponse::NotFound().body("Bot not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::AppState;
    use crate::models::Bot;
    use actix_web::{test, web, App};
    use std::collections::HashMap;
    use std::sync::Mutex;

    fn mock_app_state() -> web::Data<Mutex<AppState>> {
        let state = AppState {
            bots: HashMap::new(),
        };
        web::Data::new(Mutex::new(state))
    }

    #[actix_web::test]
    async fn test_get_bots_empty() {
        let app_state = mock_app_state();

        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(get_bots)).await;

        let req = test::TestRequest::get().uri("/bots").to_request();
        let resp = test::call_and_read_body(&app, req).await;

        // Convert the response to a Vec<u8> for comparison
        assert_eq!(resp.to_vec(), b"[]".to_vec()); // No bots initially
    }

    #[actix_web::test]
    async fn test_add_bot() {
        let app_state = mock_app_state();

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(add_bot)
                .service(get_bots),
        )
        .await;

        let bot_input = BotInput {
            name: "TestBot".to_string(),
            exchange: "Binance".to_string(),
            api_key: Some("api_key".to_string()),
            api_secret: Some("api_secret".to_string()),
            rest_endpoint: Some("http://rest_endpoint".to_string()),
            rpc_endpoint: Some("http://rpc_endpoint".to_string()),
            webhook_secret: Some("webhook_secret".to_string()),
            trading_fee: Some(0.1),
            private_key: Some("private_key".to_string()),
            contract_address: Some("contract_address".to_string()),
        };

        // Add bot
        let req = test::TestRequest::post()
            .uri("/bots")
            .set_json(&bot_input) // Serialize `bot_input` to JSON
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert!(String::from_utf8(resp.to_vec())
            .unwrap()
            .contains("Bot added successfully"));

        // Verify bot was added
        let req = test::TestRequest::get().uri("/bots").to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert!(String::from_utf8(resp.to_vec())
            .unwrap()
            .contains("TestBot"));
    }

    #[actix_web::test]
    async fn test_get_bot_by_id() {
        let app_state = mock_app_state();

        let bot_id = "123".to_string();
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
                .service(get_bot_by_id),
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/bots/{}", bot_id))
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert!(String::from_utf8(resp.to_vec())
            .unwrap()
            .contains("TestBot"));
    }

    #[actix_web::test]
    async fn test_update_bot() {
        let app_state = mock_app_state();

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(update_bot)
                .service(get_bots),
        )
        .await;

        let bot_id = "123".to_string();
        app_state.lock().unwrap().bots.insert(
            bot_id.clone(),
            Bot {
                bot_id: bot_id.clone(),
                name: "OldName".to_string(),
                exchange: "Binance".to_string(),
                api_key: None,
                api_secret: None,
                rest_endpoint: None,
                rpc_endpoint: None,
                webhook_secret: None,
                trading_fee: Some(0.1),
                private_key: None,
                contract_address: None,
                listeners: vec![],
            },
        );

        // Create the bot update input
        let bot_input = BotInput {
            name: "NewName".to_string(),
            exchange: "Binance".to_string(),
            api_key: None,
            api_secret: None,
            rest_endpoint: None,
            rpc_endpoint: None,
            webhook_secret: None,
            trading_fee: Some(0.2),
            private_key: None,
            contract_address: None,
        };

        // Send the PUT request to update the bot
        let req = test::TestRequest::put()
            .uri(&format!("/bots/{}", bot_id))
            .set_json(&bot_input)
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        assert!(std::str::from_utf8(&resp)
            .unwrap()
            .contains("Bot updated successfully"));

        // Verify the bot was updated
        {
            // Lock the app state and store it in a variable
            let locked_state = app_state.lock().unwrap();
            let updated_bot = locked_state.bots.get(&bot_id).unwrap();
            assert_eq!(updated_bot.name, "NewName");
            assert_eq!(updated_bot.trading_fee, Some(0.2));
        }
    }

    #[actix_web::test]
    async fn test_delete_bot() {
        let app_state = mock_app_state();

        let bot_id = "123".to_string();
        {
            // Insert a bot into the mock AppState
            let mut locked_state = app_state.lock().unwrap();
            locked_state.bots.insert(
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
        }

        // Initialize the Actix application with the delete_bot route
        let app =
            test::init_service(App::new().app_data(app_state.clone()).service(delete_bot)).await;

        // Send the DELETE request
        let req = test::TestRequest::delete()
            .uri(&format!("/bots/{}", bot_id))
            .to_request();
        let resp = test::call_and_read_body(&app, req).await;

        // Verify the response message
        assert!(std::str::from_utf8(&resp)
            .unwrap()
            .contains("Bot deleted successfully"));

        // Verify the bot was removed from the state
        {
            let locked_state = app_state.lock().unwrap();
            assert!(locked_state.bots.get(&bot_id).is_none());
        }
    }
}
