use message::{ChatCompletion, CreateChatCompletion};
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Url};

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
        let completion = self
            .request(Method::POST, "chat/completions")?
            .json(&payload)
            .send()
            .await?
            .json::<ChatCompletion>()
            .await?;
        Ok(completion)
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| Error::UrlParse(err.to_string()))?;
        Ok(self.http_client.request(method, url))
    }
}
