// src/errors/server.rs

use std::path::PathBuf;
use thiserror::Error;

/// Represents the different kinds of errors that can occur when loading the AppState.
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to read file {:?}: {source}", path)]
    FileReadError {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Environment variable {0} is missing or invalid")]
    EnvVarError(#[from] std::env::VarError),

    #[error("No file path provided.")]
    NoFilePathProvided,
}
