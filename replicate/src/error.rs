use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Failed to deserialize: {0}")]
    JsonDeserialization(#[from] serde_json::Error),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Missing API key {0}")]
    MissingApiKey(&'static str),

    #[error("Unexpected status code: {0}")]
    UnexpectedStatus(StatusCode),

    #[error("API error: {0}")]
    Api(ApiError),
}

#[derive(Debug, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("Error response: {status} {kind}")]
pub struct ApiError {
    /// URI that identifies the error type.
    #[serde(rename = "type")]
    pub kind: String,

    /// Short human-readable summary of the error.
    pub title: String,

    /// HTTP status code.
    pub status: u16,

    /// Human-readable explanation of the error.
    pub detail: String,

    /// URI that identifies the specific occurrence of the error.
    pub instance: String,
}
