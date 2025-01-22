use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;

/// Unified error type for the application
#[derive(Debug, Error)]
pub enum AppError {
    // API-related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unknown API error: {0}")]
    Unknown(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String), // HTTP 500

    #[error("Invalid input: {0}")]
    InvalidInput(String), // HTTP 400

    #[error("Validation error: {0}")]
    ValidationError(String), // HTTP 400

    #[error("Serialization error: {0}")]
    SerializationError(String), // HTTP 400

    #[error("Failed to connect to server: {0}")]
    ConnectionError(String), // HTTP 502

    #[error("Request timed out: {0}")]
    TimeoutError(String), // HTTP 504

    #[error("General error: {0}")]
    GeneralError(String), // HTTP 503

    #[error("Unexpected response from server: {0}")]
    UnexpectedResponse(String), // HTTP 502

    #[error("Non-unique result: {0}")]
    NonUniqueResult(String), // HTTP 400

    #[error("Argument(s) required")]
    ArgumentsRequired, // HTTP 400

    #[error("Bot ID required")]
    BotIdRequired, // HTTP 400

    #[error("Not found: {0}")]
    NotFound(String), // HTTP 404

    #[error("Bot not found: {0}")]
    BotNotFound(String), // HTTP 404

    #[error("A bot with ID `{0}` already exists.")]
    BotAlreadyExists(String), // HTTP 409

    #[error("Listener not found: {0}")]
    ListenerNotFound(String), // HTTP 404

    #[error("Failed to save state: {0}")]
    SaveError(String), // HTTP 500

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Environment variable {0} is missing or invalid.")]
    EnvVarError(#[from] std::env::VarError),

    #[error("No file path provided.")]
    NoFilePathProvided,

    #[error("Failed to read file {:?}: {source}", path)]
    FileReadError {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Failed to write file {:?}: {source}", path)]
    FileWriteError {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("State already locked. Failed to acquire lock.")]
    LockError,

    #[error("Invalid state detected: {0}")]
    InvalidState(String),

    #[error("HTTP error {0}: {1}")]
    HttpError(u16, String),
}

/// A standardized error response structure for API responses
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,
    details: Option<String>, // Optional: Provide additional context if available
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) | AppError::BotNotFound(_) | AppError::ListenerNotFound(_) => {
                StatusCode::NOT_FOUND
            }
            AppError::InvalidInput(_)
            | AppError::ValidationError(_)
            | AppError::SerializationError(_)
            | AppError::NonUniqueResult(_)
            | AppError::ArgumentsRequired
            | AppError::BotIdRequired => StatusCode::BAD_REQUEST,
            AppError::BotAlreadyExists(_) => StatusCode::CONFLICT,
            AppError::SaveError(_)
            | AppError::InternalServerError(_)
            | AppError::LockError
            | AppError::InvalidState(_)
            | AppError::FileReadError { .. }
            | AppError::FileWriteError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConnectionError(_) | AppError::UnexpectedResponse(_) => {
                StatusCode::BAD_GATEWAY
            }
            AppError::TimeoutError(_) => StatusCode::GATEWAY_TIMEOUT,
            AppError::GeneralError(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::HttpError(status, _) => {
                StatusCode::from_u16(*status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR, // Default fallback for unhandled errors
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("Error occurred: {:?}", self);

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.to_string(),
            code: self.status_code().as_u16(),
            details: None, // Optional: Add more details if applicable
        })
    }
}

/// Helper to map generic errors into `AppError::InternalServerError`
pub fn map_to_app_error<E: std::fmt::Display>(err: E) -> AppError {
    AppError::InternalServerError(err.to_string())
}

use reqwest::Error as ReqwestError;

impl From<ReqwestError> for AppError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            AppError::TimeoutError(error.to_string())
        } else if let Some(status) = error.status() {
            AppError::HttpError(status.as_u16(), format!("HTTP error: {}", status))
        } else {
            AppError::GeneralError(format!("Network or other error: {}", error))
        }
    }
}
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::InternalServerError(format!("IO error: {}", error))
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(format!("{}", err))
    }
}
