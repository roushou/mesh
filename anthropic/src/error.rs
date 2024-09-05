use std::str::Utf8Error;

use serde::Deserialize;

use crate::client::ApiVersionError;

#[derive(Debug, thiserror::Error)]
pub enum AnthropicError {
    #[error("API error: {0}")]
    Api(ApiErrorResponse),

    #[error("HTTP client error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("API version error: {0}")]
    ApiVersion(#[from] ApiVersionError),

    #[error("URL parse error: {0}")]
    UrlParse(String),

    #[error("Failed to deserialize: {0}")]
    JsonDeserialize(#[from] serde_json::Error),

    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Model not supported: {0}")]
    ModelNotSupported(String),

    #[error("Missing API key {0}")]
    MissingApiKey(&'static str),

    #[error("Invalid Stream Event")]
    InvalidStreamEvent,

    #[error("UTF8 Error: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Debug, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("Error response: {error_type} {error}")]
pub struct ApiErrorResponse {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: ApiErrorDetail,
}

#[derive(Debug, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("Api error: {error_type} {message}")]
pub struct ApiErrorDetail {
    #[serde(rename = "type")]
    pub error_type: ApiErrorType,
    pub message: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, thiserror::Error)]
pub enum ApiErrorType {
    #[error("invalid_request_error")]
    #[serde(rename = "invalid_request_error")]
    InvalidRequest,

    #[error("authentication_error")]
    #[serde(rename = "authentication_error")]
    Authentication,

    #[error("permission_error")]
    #[serde(rename = "permission_error")]
    Permission,

    #[error("not_found_error")]
    #[serde(rename = "not_found_error")]
    NotFound,

    #[error("request_too_large")]
    #[serde(rename = "request_too_large")]
    RequestTooLarge,

    #[error("rate_limit_error")]
    #[serde(rename = "rate_limit_error")]
    RateLimit,

    #[error("api_error")]
    #[serde(rename = "api_error")]
    Unexpected,

    #[error("overloaded_error")]
    #[serde(rename = "overloaded_error")]
    Overloaded,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_serialize_to_correct_error_values() {
        assert_eq!(
            &ApiErrorType::InvalidRequest.to_string(),
            "invalid_request_error"
        );
        assert_eq!(
            &ApiErrorType::Authentication.to_string(),
            "authentication_error"
        );
        assert_eq!(&ApiErrorType::Permission.to_string(), "permission_error");
        assert_eq!(&ApiErrorType::NotFound.to_string(), "not_found_error");
        assert_eq!(
            &ApiErrorType::RequestTooLarge.to_string(),
            "request_too_large"
        );
        assert_eq!(&ApiErrorType::RateLimit.to_string(), "rate_limit_error");
        assert_eq!(&ApiErrorType::Unexpected.to_string(), "api_error");
        assert_eq!(&ApiErrorType::Overloaded.to_string(), "overloaded_error");
    }
}
