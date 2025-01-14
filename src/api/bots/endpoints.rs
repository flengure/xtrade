// src/api/bots.rs

use crate::api::Pagination;
use crate::app_state::AppState;
use crate::errors::ApiError;
use crate::models::{Bot, BotInsert, BotUpdate};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use validator::Validate;

#[derive(Serialize)]
pub struct AddListenerResponse {
    pub listener_id: String,
}

#[get("/bots")]
async fn get_bots(
    data: web::Data<Mutex<AppState>>,
    query: web::Query<Pagination>,
) -> Result<impl Responder, ApiError> {
    let state = data.lock().unwrap();
    let bots: Vec<Bot> = state.list_bots();

    // Determine if pagination parameters are provided
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(bots.len()); // If limit is not provided, return all bots

    // Calculate start and end indices
    let start = (page - 1) * limit;
    let end = start + limit;

    // Handle cases where start exceeds the number of bots
    if start >= bots.len() {
        return Ok(HttpResponse::Ok().json(ApiResponse::<Vec<&Bot>> {
            success: true,
            data: Some(Vec::new()),
            error: None,
        }));
    }

    // Adjust end index if it exceeds the number of bots
    let end = if end > bots.len() { bots.len() } else { end };

    // Slice the bots vector to get the paginated subset
    let paginated_bots = bots[start..end].to_vec();

    // Prepare the response
    let api_response = ApiResponse {
        success: true,
        data: Some(paginated_bots),
        error: None,
    };

    Ok(HttpResponse::Ok().json(api_response))
}

#[derive(Serialize)]
pub struct AddBotResponse {
    pub bot_id: String,
    pub name: String,
    pub exchange: String,
}

// Unified APIResponse Struct
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[post("/bots")]
async fn add_bot(
    data: web::Data<Mutex<AppState>>,
    bot_input: web::Json<BotInsert>,
) -> Result<impl Responder, ApiError> {
    let mut state = data.lock().unwrap();

    // Validate the request
    if let Err(validation_errors) = bot_input.validate() {
        return Err(ApiError::InvalidInput(validation_errors.to_string()));
    }

    let bot = state.create_bot(bot_input.into_inner())?;

    let api_response = ApiResponse {
        success: true,
        data: Some(bot.clone()),
        error: None,
    };

    // Prepare the Location header
    let location = format!("/bots/{}", bot.bot_id);

    // Return 201 Created with Location header and JSON body
    Ok(HttpResponse::Created()
        .insert_header(("Location", location))
        .json(api_response))
}

#[get("/bots/{bot_id}")]
async fn get_bot_by_id(
    data: web::Data<Mutex<AppState>>,
    bot_id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let state = data.lock().unwrap();
    let bot_id = bot_id.into_inner();

    if let Some(bot) = state.get_bot(&bot_id) {
        let api_response = ApiResponse {
            success: true,
            data: Some(bot),
            error: None,
        };
        Ok(HttpResponse::Ok().json(api_response))
    } else {
        Err(ApiError::BotNotFound)
    }
}

#[put("/bots/{bot_id}")]
async fn update_bot(
    data: web::Data<Mutex<AppState>>,
    bot_id: web::Path<String>,
    bot_input: web::Json<BotUpdate>,
) -> Result<impl Responder, ApiError> {
    // Acquire the lock on AppState
    let mut state = data.lock().map_err(|_| ApiError::InternalServerError)?;

    // Extract and update the bot ID in the input
    let mut updated_bot = bot_input.into_inner();
    updated_bot.bot_id = bot_id.into_inner();

    // Update the bot in the state
    match state.update_bot(updated_bot) {
        Ok(bot) => {
            // Create the success response
            let api_response = ApiResponse {
                success: true,
                data: Some(bot),
                error: None,
            };
            Ok(HttpResponse::Ok().json(api_response))
        }
        Err(e) => Err(e), // Pass through the error
    }
}

#[delete("/bots/{bot_id}")]
async fn delete_bot(
    data: web::Data<Mutex<AppState>>,
    bot_id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let mut state = data.lock().unwrap();

    if state.bots.remove(&bot_id.into_inner()).is_some() {
        // Persist the state
        state.save_state_to_file("state.json");

        let api_response = ApiResponse::<String> {
            success: true,
            data: Some("Bot deleted successfully".into()),
            error: None,
        };

        Ok(HttpResponse::Ok().json(api_response))
    } else {
        Err(ApiError::BotNotFound)
    }
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
