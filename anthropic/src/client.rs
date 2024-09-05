use core::fmt;
use futures_util::{stream, Stream, StreamExt};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Method, RequestBuilder, Url,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{
    completion::{
        message::{MessageRequest, MessageResponse},
        stream::StreamEvent,
    },
    config::Config,
    error::{AnthropicError, ApiErrorResponse},
};

const ANTHROPIC_API_KEY_HEADER: &str = "x-api-key";
const ANTHROPIC_BETA_HEADERS: &str = "anthropic-beta";
const ANTHROPIC_VERSION_HEADER: &str = "anthropic-version";

pub struct Client {
    api_key: String,
    api_version: ApiVersion,
    anthropic_version: AnthropicVersion,
    base_url: Url,
    beta_headers: Option<String>,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, AnthropicError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ANTHROPIC_API_KEY_HEADER,
            HeaderValue::from_str(config.api_key.as_str())
                .map_err(AnthropicError::InvalidHeaderValue)?,
        );
        headers.insert(
            ANTHROPIC_VERSION_HEADER,
            HeaderValue::from_str(&config.anthropic_version.to_string())
                .map_err(AnthropicError::InvalidHeaderValue)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let base_url = Url::parse(&config.base_url)
            .map_err(|err| AnthropicError::UrlParse(err.to_string()))?
            .join(format!("{}/", config.api_version).as_str())
            .map_err(|err| AnthropicError::UrlParse(err.to_string()))?;

        Ok(Self {
            anthropic_version: config.anthropic_version,
            api_key: config.api_key,
            api_version: config.api_version,
            base_url,
            beta_headers: None,
            http_client,
        })
    }

    pub fn anthropic_version(&self) -> &AnthropicVersion {
        &self.anthropic_version
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    pub fn api_version(&self) -> &ApiVersion {
        &self.api_version
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    pub fn beta_headers(&self) -> Option<&str> {
        self.beta_headers.as_deref()
    }

    pub fn with_beta_header(&mut self, new_header: &str) {
        if let Some(existing_headers) = &self.beta_headers {
            self.beta_headers = Some(format!("{},{}", existing_headers, new_header));
        } else {
            self.beta_headers = Some(new_header.to_string());
        }
    }

    pub fn with_beta_headers(&mut self, headers: &[&str]) {
        if let Some(existing_headers) = &self.beta_headers {
            let new_headers = headers.join(",");
            self.beta_headers = Some(format!("{},{}", existing_headers, new_headers));
        } else {
            self.beta_headers = Some(headers.join(","));
        }
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, AnthropicError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| AnthropicError::UrlParse(err.to_string()))?;
        let req = self.http_client.request(method, url);
        if let Some(beta_headers) = &self.beta_headers {
            Ok(req.header(ANTHROPIC_BETA_HEADERS, beta_headers))
        } else {
            Ok(req)
        }
    }

    pub async fn create_message(
        &self,
        payload: MessageRequest,
    ) -> Result<MessageResponse, AnthropicError> {
        let response = self
            .request(Method::POST, "messages")?
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            match serde_json::from_str::<ApiErrorResponse>(&error) {
                Ok(api_error) => return Err(AnthropicError::Api(api_error)),
                Err(err) => return Err(AnthropicError::JsonDeserialize(err)),
            }
        }

        response
            .json::<MessageResponse>()
            .await
            .map_err(AnthropicError::from)
    }

    pub async fn stream_message(
        &self,
        request: MessageRequest,
    ) -> Result<impl Stream<Item = Result<StreamEvent, AnthropicError>>, AnthropicError> {
        let response = self
            .request(Method::POST, "messages")?
            .header(ACCEPT, "text/event-stream")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            match serde_json::from_str::<ApiErrorResponse>(&error) {
                Ok(api_error) => return Err(AnthropicError::Api(api_error)),
                Err(err) => return Err(AnthropicError::JsonDeserialize(err)),
            }
        }

        Ok(response.bytes_stream().flat_map(move |chunk| match chunk {
            Ok(bytes) => {
                let events = Self::parse_stream_chunk(&bytes);
                stream::iter(events)
            }
            Err(err) => stream::iter(vec![Err(AnthropicError::from(err))]),
        }))
    }

    fn parse_stream_chunk(bytes: &[u8]) -> Vec<Result<StreamEvent, AnthropicError>> {
        let chunk_str = match std::str::from_utf8(bytes).map_err(AnthropicError::Utf8Error) {
            Ok(chunk_str) => chunk_str,
            Err(err) => return vec![Err(err)],
        };
        chunk_str
            .split("\n\n")
            .filter(|event| !event.trim().is_empty())
            .map(|event| {
                event
                    .lines()
                    .find(|line| line.starts_with("data: "))
                    .and_then(|line| line.strip_prefix("data: "))
                    .ok_or(AnthropicError::InvalidStreamEvent)
                    .and_then(|content| {
                        StreamEvent::from_str(content)
                            .map_err(|_| AnthropicError::InvalidStreamEvent)
                    })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnthropicVersion {
    Latest,
    Initial,
}

impl Default for AnthropicVersion {
    fn default() -> Self {
        Self::Latest
    }
}

impl fmt::Display for AnthropicVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Latest => write!(f, "2023-06-01"),
            Self::Initial => write!(f, "2023-01-01"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiVersion {
    V1,
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::V1
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V1 => write!(f, "v1"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Invalid API version: {0}")]
pub struct ApiVersionError(String);

impl FromStr for ApiVersion {
    type Err = ApiVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            _ => Err(ApiVersionError(s.to_string())),
        }
    }
}
