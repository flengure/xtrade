// src/errors/server.rs
use super::ApiError;
use std::path::PathBuf;
use thiserror::Error;

#[allow(dead_code)]
pub fn map_to_server_error<E: std::fmt::Display>(err: E) -> ServerError {
    ServerError::Other(format!("{}", err))
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("API error: {0}")]
    ApiError(#[from] Box<ApiError>),

    #[allow(dead_code)]
    #[error("Other error: {0}")]
    Other(String),

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

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Environment variable {0} is missing or invalid.")]
    EnvVarError(#[from] std::env::VarError),

    #[error("No file path provided.")]
    NoFilePathProvided,

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[allow(dead_code)]
    #[error("State already locked. Failed to acquire lock.")]
    LockError,

    #[allow(dead_code)]
    #[error("Invalid state detected: {0}")]
    InvalidState(String),
}
