//! Error type used throughout the client.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("config: {0}")]
    Config(String),

    #[error("authentication: {0}")]
    Auth(String),

    #[error("http: {0}")]
    Http(String),

    #[error("not found")]
    NotFound,

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}
