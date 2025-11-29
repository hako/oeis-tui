#![allow(dead_code)]

use thiserror::Error;

/// Application-specific errors
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("OEIS API error: {0}")]
    OEISApi(String),

    #[error("Sequence not found: {0}")]
    SequenceNotFound(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Clipboard error: {0}")]
    Clipboard(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

/// Result type using AppError
pub type Result<T> = std::result::Result<T, AppError>;
