use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(String),

    #[error("Too Many Requests: {0}")]
    TooManyRequests(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Bad Gateway: {0}")]
    BadGateway(String),

    #[error("Service Unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Missing api key: {0}")]
    MissingApiKey(&'static str),

    #[error("URL parse error: {0}")]
    UrlParse(String),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Deserialization Error: {0}")]
    Deserialization(String),

    #[error("Unexpected Error: {0}")]
    Unexpected(StatusCode),
}
