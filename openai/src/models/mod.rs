use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub mod gpt;

pub struct ModelClient {
    base_url: Url,
    http_client: ReqwestClient,
}

impl ModelClient {
    pub fn new(base_url: Url, http_client: ReqwestClient) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    pub async fn get_model(&self, model_id: impl Into<String>) -> Result<Model, Error> {
        let path = format!("models/{}", model_id.into());
        let model = self
            .request(Method::GET, path.as_str())?
            .send()
            .await?
            .json::<Model>()
            .await?;
        Ok(model)
    }

    pub async fn list_models(&self) -> Result<ListModelsResponse, Error> {
        let models = self
            .request(Method::GET, "models")?
            .send()
            .await?
            .json::<ListModelsResponse>()
            .await?;
        Ok(models)
    }

    pub async fn delete_model(
        &self,
        model_id: impl Into<String>,
    ) -> Result<DeleteModelResponse, Error> {
        let path = format!("models/{}", model_id.into());
        let response = self
            .request(Method::DELETE, path.as_str())?
            .send()
            .await?
            .json::<DeleteModelResponse>()
            .await?;
        Ok(response)
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| Error::UrlParse(err.to_string()))?;
        Ok(self.http_client.request(method, url))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// The model id.
    pub id: String,

    /// The object type, which is always "model".
    pub object: String,

    /// The Unix timestamp (in seconds) when the model was created.
    pub created: u64,

    /// The organization that owns the model.
    pub owned_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
