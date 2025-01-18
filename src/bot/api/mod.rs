// src/bot/api/mod.rs
pub mod bots;
pub mod listeners;
pub use crate::bot::state::{
    AppState, BotInsertArgs, BotListArgs, BotListView, BotUpdateArgs, ListenerInsertArgs,
    ListenerListArgs,
};
pub use crate::errors::ApiError;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// /// Initialize all routes for the API
// pub fn init_routes(cfg: &mut web::ServiceConfig) {
//     bots::configure(cfg);
//     listeners::configure(cfg);
// }

/// Unified API response structure.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Helper to acquire a lock on `AppState`.
pub fn acquire_lock(
    data: &web::Data<Arc<Mutex<AppState>>>,
) -> Result<std::sync::MutexGuard<AppState>, ApiError> {
    data.lock().map_err(|e| {
        log::error!("Failed to acquire lock on AppState: {}", e);
        ApiError::InternalServerError("Failed to acquire lock on AppState".to_string())
    })
}

/// Helper to apply pagination.
pub fn apply_pagination<T>(data: &[T], page: usize, limit: usize) -> Vec<T>
where
    T: Clone,
{
    let start = (page - 1) * limit;
    let end = start + limit;
    if start < data.len() {
        data[start..data.len().min(end)].to_vec()
    } else {
        Vec::new()
    }
}

/// Helper to create an API response.
pub fn create_api_response<T>(
    success: bool,
    data: Option<T>,
    error: Option<String>,
) -> ApiResponse<T> {
    ApiResponse {
        success,
        data,
        error,
    }
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,  // Optional: Defaults to `Some(1)`
    pub limit: Option<usize>, // Optional: Defaults to `Some(10)`
}

impl Pagination {
    /// Get the effective page number (defaults to 1)
    pub fn page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    /// Get the effective limit (defaults to 10)
    pub fn limit(&self) -> usize {
        self.limit.unwrap_or(10)
    }

    /// Validate pagination parameters
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.page().max(1) < 1 {
            return Err(ApiError::InvalidInput(
                "Page number must be greater than or equal to 1.".to_string(),
            ));
        }
        if self.limit().max(1) < 1 {
            return Err(ApiError::InvalidInput(
                "Limit must be greater than or equal to 1.".to_string(),
            ));
        }
        Ok(())
    }
}
