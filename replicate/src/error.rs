use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API error: {0}")]
    Api(ApiError),

    #[error("HTTP client error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(String),

    #[error("Failed to deserialize: {0}")]
    JsonDeserialize(#[from] serde_json::Error),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Missing API key {0}")]
    MissingApiKey(&'static str),

    #[error("Invalid Stream Event")]
    InvalidStreamEvent,

    #[error("Unexpected error: {0}")]
    Unexpected(String),
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
