use core::fmt;
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::error::Error;

pub mod gpt;
pub mod o1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    Gpt(gpt::Gpt),
    O1(o1::O1),
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Model::Gpt(gpt) => write!(f, "{}", gpt),
            Model::O1(o1) => write!(f, "{}", o1),
        }
    }
}

impl FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(gpt) = s.parse::<gpt::Gpt>() {
            Ok(Model::Gpt(gpt))
        } else if let Ok(o1) = s.parse::<o1::O1>() {
            Ok(Model::O1(o1))
        } else {
            Err(format!("Invalid model string: {}", s))
        }
    }
}

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

    pub async fn get_model(&self, model_id: impl Into<String>) -> Result<ModelInfo, Error> {
        let path = format!("models/{}", model_id.into());
        let model = self
            .request(Method::GET, path.as_str())?
            .send()
            .await?
            .json::<ModelInfo>()
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
pub struct ModelInfo {
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
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteModelResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
