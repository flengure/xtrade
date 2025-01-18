// src/errors.rs

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
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
    InternalServerError(String), // HTTP 500

    #[error("Invalid input: {0}")]
    InvalidInput(String), // HTTP 400

    #[error("Validation error: {0}")]
    ValidationError(String), // HTTP 400

    #[error("Serialization error: {0}")]
    SerializationError(String), // HTTP 400

    #[error("Failed to connect to server")]
    ConnectionError(String), // Client-side error

    #[error("Request timed out")]
    TimeoutError(String), // Client-side error

    #[error("General error")]
    GeneralError(String), // Client-side error

    #[error("Unexpected response from server: {0}")]
    UnexpectedResponse(String), // Could represent a non-500 server error

    #[error("Non Unique Result {0}")]
    NonUniqueResult(String), // HTTP 400

    #[error("Argument(s) required")]
    ArgumentsRequired, // HTTP 400

    #[error("Bot ID required")]
    BotIdRequired, // HTTP 400

    // Bot-related errors
    #[error("Not found: {0}")]
    NotFound(String), // HTTP 404

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

    #[error("Failed to parse response: {0}")]
    ParseError(String), // Client-side error

    #[error("HTTP error {0}: {1}")]
    HttpError(u16, String), // Represents unexpected HTTP status codes
}

/// A standardized error response structure.
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: u16,               // Optional: Include the HTTP status code in the response
    details: Option<String>, // Optional: Provide additional context if available
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND, // HTTP 404
            ApiError::BotNotFound(_) => actix_web::http::StatusCode::NOT_FOUND, // HTTP 404
            ApiError::ListenerNotFound(_) => actix_web::http::StatusCode::NOT_FOUND, // HTTP 404
            ApiError::InvalidInput(_) => actix_web::http::StatusCode::BAD_REQUEST, // HTTP 400
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST, // HTTP 400
            ApiError::SerializationError(_) => actix_web::http::StatusCode::BAD_REQUEST, // HTTP 400
            ApiError::NonUniqueResult(_) => actix_web::http::StatusCode::BAD_REQUEST, // HTTP 400
            ApiError::ArgumentsRequired => actix_web::http::StatusCode::BAD_REQUEST, // HTTP 400
            ApiError::BotIdRequired => actix_web::http::StatusCode::BAD_REQUEST, // HTTP 400
            ApiError::BotAlreadyExists(_) => actix_web::http::StatusCode::CONFLICT, // HTTP 409
            ApiError::InsertionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // HTTP 500
            ApiError::SaveError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // HTTP 500
            ApiError::InternalServerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, // HTTP 500
            ApiError::ConnectionError(_) => actix_web::http::StatusCode::BAD_GATEWAY, // HTTP 502
            ApiError::TimeoutError(_) => actix_web::http::StatusCode::GATEWAY_TIMEOUT, // HTTP 504
            ApiError::GeneralError(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE, // HTTP 503
            ApiError::UnexpectedResponse(_) => actix_web::http::StatusCode::BAD_GATEWAY, // HTTP 502
            ApiError::ParseError(_) => StatusCode::BAD_REQUEST, // HTTP 400 (parsing is usually client-related)
            ApiError::HttpError(status, _) => {
                StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            } // Use the status code provided
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

use reqwest::Error as ReqwestError;

impl From<ReqwestError> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            // Convert timeout errors to a specific ApiError
            ApiError::TimeoutError(error.to_string())
        } else if let Some(status) = error.status() {
            // Use the HTTP status code from the error if available
            ApiError::HttpError(status.as_u16(), format!("HTTP error: {}", status))
        } else {
            // General error handling for all other types of errors
            ApiError::GeneralError(format!("Network or other error: {}", error))
        }
    }
}
