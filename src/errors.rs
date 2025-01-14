// src/errors.rs

use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;

/// Represents the different kinds of errors that can occur when loading the AppState.
#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Failed to read file {:?}: {source}", path)]
    FileReadError {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Environment variable STATE_FILE is invalid: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("No file path provided and AppState's file field is None.")]
    NoFilePathProvided,
}

#[derive(Debug, Error)]
pub enum ApiError {
    // General errors
    #[error("Internal server error")]
    InternalServerError, // HTTP 500

    #[error("Invalid input: {0}")]
    InvalidInput(String), // HTTP 400

    #[error("Validation error: {0}")]
    ValidationError(String), // HTTP 400

    #[error("Non Unique Result {0}")]
    NonUniqueResult(String), // HTTP 400

    #[error("Argument(s) required")]
    ArgumentsRequired, // HTTP 400

    #[error("Bot ID required")]
    BotIdRequired, // HTTP 400

    // Bot-related errors
    #[error("Bot not found: {0}")]
    BotNotFound(String), // HTTP 404

    #[error("A bot with ID `{0}` already exists.")]
    BotAlreadyExists(String), // HTTP 409

    #[error("Failed to add the bot due to an internal error.")]
    InsertionError, // HTTP 500

    #[error("Failed to save state: {0}")]
    SaveError(String), // HTTP 500

    // Listener-related errors
    #[error("Listener not found: {0}")]
    ListenerNotFound(String), // HTTP 404
}

/// A standardized error response structure.
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiError::BotNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::ListenerNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            ApiError::InvalidInput(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::NonUniqueResult(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::ArgumentsRequired => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::BotIdRequired => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::BotAlreadyExists(_) => actix_web::http::StatusCode::CONFLICT,
            ApiError::InsertionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SaveError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: error_message,
        })
    }
}
