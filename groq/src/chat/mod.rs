use message::{ChatCompletion, CreateChatCompletion};
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response, StatusCode, Url};

use crate::error::Error;

pub mod message;

pub struct ChatClient {
    base_url: Url,
    http_client: ReqwestClient,
}

impl ChatClient {
    pub fn new(base_url: Url, http_client: ReqwestClient) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    pub async fn create_completion(
        &self,
        payload: CreateChatCompletion,
    ) -> Result<ChatCompletion, Error> {
        let response = self
            .request(Method::POST, "chat/completions")?
            .json(&payload)
            .send()
            .await?;
        self.handle_response::<ChatCompletion>(response).await
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
