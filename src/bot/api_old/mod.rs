// src/bot/api/mod.rs
pub mod endpoints;
//pub mod listeners;
pub use crate::bot::state::input::bot::{BotDeleteArgs, BotInsertArgs, BotListArgs, BotUpdateArgs};
pub use crate::bot::state::input::listener::{
    ListenerDeleteArgs, ListenerGetArgs, ListenerInsertArgs, ListenerListArgs, ListenerUpdateArgs,
    ListenersDeleteArgs,
};
pub use crate::bot::state::output::bot::BotListView;
pub use crate::bot::state::AppState;
pub use crate::errors::AppError;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Unified API response structure.
#[derive(Deserialize, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Helper to acquire a lock on `AppState`.
pub fn acquire_lock(
    data: &web::Data<Arc<Mutex<AppState>>>,
) -> Result<std::sync::MutexGuard<AppState>, AppError> {
    data.lock().map_err(|e| {
        log::error!("Failed to acquire lock on AppState: {}", e);
        AppError::InternalServerError("Failed to acquire lock on AppState".to_string())
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
    pub fn validate(&self) -> Result<(), AppError> {
        if self.page().max(1) < 1 {
            return Err(AppError::InvalidInput(
                "Page number must be greater than or equal to 1.".to_string(),
            ));
        }
        if self.limit().max(1) < 1 {
            return Err(AppError::InvalidInput(
                "Limit must be greater than or equal to 1.".to_string(),
            ));
        }
        Ok(())
    }
}
