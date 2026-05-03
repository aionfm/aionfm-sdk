use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

pub type AionFmResult<T> = Result<T, AionFmError>;

/// SDK error enum covering network, serialization, API, and configuration failures.
#[derive(Debug, Error)]
pub enum AionFmError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error("api error: status {status}: {message}")]
    Api { status: u16, message: String },
    #[error("unexpected response: {0}")]
    Unexpected(String),
}
