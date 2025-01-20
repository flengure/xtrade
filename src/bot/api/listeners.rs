// src/bot/api/listeners.rs
use crate::bot::state::{
    acquire_lock, create_api_response, ApiError, AppState, ListenerGetArgs, ListenerInsertArgs,
    ListenerListArgs,
};
use actix_web::{delete, post, web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

/// Configure listener-related API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(add_listener)
        .service(delete_listeners)
        .service(delete_listener);
}

#[post("/bots/{bot_id}/listeners")]
async fn add_listener(
    data: web::Data<Arc<Mutex<AppState>>>,
    bot_id: web::Path<String>,
    json_data: Result<web::Json<ListenerInsertArgs>, actix_web::Error>,
) -> Result<impl Responder, ApiError> {
    let mut state = acquire_lock(&data)?;
    match json_data {
        Ok(good_json_data) => {
            let listener = state.add_listener(&bot_id.into_inner(), good_json_data.into_inner())?;
            let api_response = create_api_response(true, Some(listener), None);
            Ok(HttpResponse::Ok().json(api_response))
        }
        Err(e) => {
            log::error!("Failed to deserialize input: {}", e);
            Err(ApiError::InvalidInput("Invalid input payload".to_string()))
        }
    }
}

#[delete("/bots/{bot_id}/listeners")]
async fn delete_listeners(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<String>,
    json_data: Option<web::Json<ListenerListArgs>>,
) -> Result<impl Responder, ApiError> {
    let bot_id = path.into_inner();
    let delete_request = json_data
        .map(|payload| payload.into_inner())
        .unwrap_or_else(|| ListenerListArgs {
            listener_id: None,
            service: None,
        });

    let mut state = acquire_lock(&data)?;
    let deleted_list = state.delete_listeners(&bot_id, delete_request)?;
    drop(state);

    if deleted_list.0.is_empty() {
        return Err(ApiError::ListenerNotFound(
            "No matching listeners found.".to_string(),
        ));
    }

    let api_response = create_api_response(true, Some(deleted_list), None);
    Ok(HttpResponse::Ok().json(api_response))
}

#[delete("/bots/{bot_id}/listeners/{listener_id}")]
async fn delete_listener(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, ApiError> {
    let (bot_id, listener_id) = path.into_inner();
    let mut state = acquire_lock(&data)?;
    state.delete_listener(&bot_id, &listener_id)?;
    let api_response = create_api_response(
        true,
        Some("Listener deleted successfully".to_string()),
        None,
    );
    Ok(HttpResponse::Ok().json(api_response))
}
