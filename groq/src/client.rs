use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client as ReqwestClient, Method, RequestBuilder, Response, StatusCode, Url,
};
use serde::{Deserialize, Serialize};

use crate::{chat::ChatClient, config::Config, error::Error};

pub struct Client {
    api_key: String,
    base_url: Url,
    pub chat: ChatClient,
    http_client: ReqwestClient,
}

impl Client {
    pub fn new(config: Config) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", config.api_key.as_str()).as_str())?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let base_url =
            Url::parse(&config.base_url).map_err(|err| Error::UrlParse(err.to_string()))?;

        Ok(Self {
            api_key: config.api_key,
            base_url: base_url.clone(),
            chat: ChatClient::new(base_url, http_client.clone()),
            http_client,
        })
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    pub async fn list_models(&self) -> Result<ListModelsResponse, Error> {
        let models = self.request(Method::GET, "models")?.send().await?;
        self.handle_response::<ListModelsResponse>(models).await
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| Error::UrlParse(err.to_string()))?;
        Ok(self.http_client.request(method, url))
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        if status.is_success() | status.is_redirection() {
            match response.json::<T>().await {
                Ok(data) => Ok(data),
                // TODO: this should be a serde error
                Err(err) => Err(Error::HttpRequest(err)),
            }
        } else {
            match status {
                StatusCode::UNAUTHORIZED => {
                    let error_msg = response.text().await?;
                    Err(Error::Unauthorized(error_msg))
                }
                StatusCode::BAD_REQUEST => {
                    let error_msg = response.text().await?;
                    Err(Error::BadRequest(error_msg))
                }
                StatusCode::FORBIDDEN => {
                    let error_msg = response.text().await?;
                    Err(Error::Forbidden(error_msg))
                }
                StatusCode::UNPROCESSABLE_ENTITY => {
                    let error_msg = response.text().await?;
                    Err(Error::UnprocessableEntity(error_msg))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    let error_msg = response.text().await?;
                    Err(Error::RateLimited(error_msg))
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    let error_msg = response.text().await?;
                    Err(Error::InternalServerError(error_msg))
                }
                StatusCode::BAD_GATEWAY => {
                    let error_msg = response.text().await?;
                    Err(Error::BadGateway(error_msg))
                }
                StatusCode::SERVICE_UNAVAILABLE => {
                    let error_msg = response.text().await?;
                    Err(Error::ServiceUnavailable(error_msg))
                }
                status => Err(Error::Unexpected(status)),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsResponse {
    data: Vec<ModelMetadata>,
    object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    active: bool,
    context_window: u32,
    created: u64,
    id: String,
    object: String,
    owned_by: String,
    public_apps: Option<serde_json::Value>,
}
