// src/bot/api/bots.rs
pub use crate::bot::state::BotGetArgs;
use crate::errors::ApiError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};

use crate::bot::api::{acquire_lock, apply_pagination, create_api_response, Pagination};

use crate::bot::state::{
    AppState, BotDeleteArgs, BotInsertArgs, BotListArgs, BotListView, BotUpdateArgs,
    ListenerDeleteArgs, ListenerGetArgs, ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs,
    ListenersDeleteArgs,
};

/// Configure bot-related API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(add_bot)
        .service(get_bots)
        .service(get_bot)
        .service(update_bot)
        .service(delete_bot)
        .service(add_listener)
        .service(list_listeners)
        .service(get_listener)
        .service(update_listener)
        .service(delete_listeners)
        .service(delete_listener);
}

#[post("/bots")]
async fn add_bot(
    data: web::Data<Arc<Mutex<AppState>>>,
    json_data: Result<web::Json<BotInsertArgs>, actix_web::Error>,
) -> Result<impl Responder, ApiError> {
    let mut state = acquire_lock(&data)?;
    match json_data {
        Ok(good_json_data) => {
            let bot = state.add_bot(good_json_data.into_inner())?;
            let location = format!("/bots/{}", bot.bot_id);
            let api_response = create_api_response(true, Some(bot.clone()), None);

            Ok(HttpResponse::Created()
                .insert_header(("Location", location))
                .json(api_response))
        }
        Err(e) => {
            log::error!("Failed to deserialize input: {}", e);
            Err(ApiError::InvalidInput("Invalid input payload".to_string()))
        }
    }
}

#[get("/bots")]
async fn get_bots(
    data: web::Data<Arc<Mutex<AppState>>>,
    query: Option<web::Query<Pagination>>, // Pagination query is optional
    filter: Option<web::Json<BotListArgs>>, // Optional filter in the request body
) -> Result<impl Responder, ApiError> {
    // Use default pagination values if none are provided
    let pagination = query.unwrap_or_else(|| {
        web::Query(Pagination {
            page: Some(1),
            limit: Some(10),
        })
    });

    // Validate the pagination parameters
    pagination.validate()?;

    // Acquire the state lock
    let state = acquire_lock(&data)?;

    // Extract optional filter arguments
    let filter_args = filter.map(|f| f.into_inner());

    // Fetch the list of bots
    let bots = state.list_bots(filter_args)?;
    drop(state); // Release the state lock early

    // Apply pagination
    let paginated_bots = apply_pagination(&bots.0, pagination.page(), pagination.limit());

    // If no bots exist, return an empty response
    if paginated_bots.is_empty() {
        return Ok(HttpResponse::Ok().json(create_api_response::<BotListView>(
            true,
            Some(BotListView(vec![])),
            None,
        )));
    }

    // Construct and return the paginated response
    let response = create_api_response(true, Some(BotListView(paginated_bots)), None);
    Ok(HttpResponse::Ok().json(response))
}

#[get("/bots/{bot_id}")]
async fn get_bot(
    data: web::Data<Arc<Mutex<AppState>>>,
    bot_id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let state = acquire_lock(&data)?;
    let bot = state.get_bot(BotGetArgs::new(&bot_id))?;
    let api_response = create_api_response(true, Some(bot), None);
    Ok(HttpResponse::Ok().json(api_response))
}

#[put("/bots/{bot_id}")]
async fn update_bot(
    data: web::Data<Arc<Mutex<AppState>>>,
    bot_id: web::Path<String>,
    json_data: Result<web::Json<BotUpdateArgs>, actix_web::Error>,
) -> Result<impl Responder, ApiError> {
    let mut state = acquire_lock(&data)?;
    match json_data {
        Ok(good_json_data) => {
            let mut update_data = good_json_data.into_inner();
            update_data.bot_id = bot_id.to_string();
            let bot = state.update_bot(update_data)?;
            let api_response = create_api_response(true, Some(bot), None);
            Ok(HttpResponse::Ok().json(api_response))
        }
        Err(e) => {
            log::error!("Failed to deserialize input: {}", e);
            Err(ApiError::InvalidInput("Invalid input payload".to_string()))
        }
    }
}

#[delete("/bots/{bot_id}")]
async fn delete_bot(
    data: web::Data<Arc<Mutex<AppState>>>,
    bot_id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let mut state = acquire_lock(&data)?;
    state.delete_bot(BotDeleteArgs::new(&bot_id))?;
    let api_response =
        create_api_response(true, Some("Bot deleted successfully".to_string()), None);
    Ok(HttpResponse::Ok().json(api_response))
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
            let mut args = good_json_data.into_inner();
            args.bot_id = bot_id.into_inner();
            let listener = state.add_listener(args)?;
            let api_response = create_api_response(true, Some(listener), None);
            Ok(HttpResponse::Ok().json(api_response))
        }
        Err(e) => {
            log::error!("Failed to deserialize input: {}", e);
            Err(ApiError::InvalidInput("Invalid input payload".to_string()))
        }
    }
}

#[get("/bots/{bot_id}/listeners")]
async fn list_listeners(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<String>,
    json_data: Option<web::Json<ListenerListArgs>>,
) -> Result<impl Responder, ApiError> {
    let bot_id = path.into_inner();
    let mut select_request = json_data
        .map(|payload| payload.into_inner())
        .unwrap_or_else(|| ListenerListArgs::new(bot_id.clone()))
        .bot_id(bot_id.clone());

    select_request.bot_id = bot_id;
    let state = acquire_lock(&data)?;
    let selected_list = state.list_listeners(select_request)?;
    drop(state);

    if selected_list.0.is_empty() {
        return Err(ApiError::ListenerNotFound(
            "No matching listeners found.".to_string(),
        ));
    }

    let api_response = create_api_response(true, Some(selected_list), None);
    Ok(HttpResponse::Ok().json(api_response))
}

#[get("/bots/{bot_id}/listeners/{listener_id}")]
async fn get_listener(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, ApiError> {
    let (bot_id, listener_id) = path.into_inner();
    let state = acquire_lock(&data)?;

    state.get_listener(ListenerGetArgs::new(&bot_id, &listener_id))?;
    let api_response = create_api_response(
        true,
        Some("Listener deleted successfully".to_string()),
        None,
    );
    Ok(HttpResponse::Ok().json(api_response))
}

#[put("/bots/{bot_id}/listeners/{listener_id}")]
async fn update_listener(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, ApiError> {
    let (bot_id, listener_id) = path.into_inner();
    let mut state = acquire_lock(&data)?;
    state.update_listener(ListenerUpdateArgs::new(&bot_id, &listener_id))?;
    let api_response = create_api_response(
        true,
        Some("Listener deleted successfully".to_string()),
        None,
    );
    Ok(HttpResponse::Ok().json(api_response))
}

#[delete("/bots/{bot_id}/listeners/{listener_id}")]
async fn delete_listener(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, ApiError> {
    let (bot_id, listener_id) = path.into_inner();
    let mut state = acquire_lock(&data)?;
    //let args = ListenerDeleteArgs::new(bot_id, listener_id);

    state.delete_listener(ListenerDeleteArgs::new(&bot_id, &listener_id))?;
    let api_response = create_api_response(
        true,
        Some("Listener deleted successfully".to_string()),
        None,
    );
    Ok(HttpResponse::Ok().json(api_response))
}

#[delete("/bots/{bot_id}/listeners")]
async fn delete_listeners(
    data: web::Data<Arc<Mutex<AppState>>>,
    path: web::Path<String>,
    json_data: Option<web::Json<ListenersDeleteArgs>>,
) -> Result<impl Responder, ApiError> {
    let bot_id = path.into_inner();
    let mut delete_request = json_data
        .map(|payload| payload.into_inner())
        .unwrap_or_else(|| ListenersDeleteArgs::new(bot_id.clone()))
        .bot_id(bot_id.clone());

    delete_request.bot_id = bot_id;
    let mut state = acquire_lock(&data)?;
    let deleted_list = state.delete_listeners(delete_request)?;
    drop(state);

    if deleted_list.0.is_empty() {
        return Err(ApiError::ListenerNotFound(
            "No matching listeners found.".to_string(),
        ));
    }

    let api_response = create_api_response(true, Some(deleted_list), None);
    Ok(HttpResponse::Ok().json(api_response))
}
