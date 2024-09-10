use core::fmt;
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub struct EmbeddingClient {
    base_url: Url,
    http_client: ReqwestClient,
}

impl EmbeddingClient {
    pub fn new(base_url: Url, http_client: ReqwestClient) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    pub async fn create_embedding(
        &self,
        payload: CreateEmbedding,
    ) -> Result<CreateEmbeddingResponse, Error> {
        let embedding = self
            .request(Method::POST, "embeddings")?
            .json(&payload)
            .send()
            .await?
            .json::<CreateEmbeddingResponse>()
            .await?;
        Ok(embedding)
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| Error::UrlParse(err.to_string()))?;
        Ok(self.http_client.request(method, url))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embedding {
    /// The index of the embedding in the list of embeddings.
    pub index: u64,

    /// The embedding vector represented as a list of floats.
    ///
    /// The length of vector depends on the model:
    ///     - **text-embedding-3-large**: 3,072
    ///     - **text-embedding-3-small**: 1,536
    ///     - **text-embedding-ada-002**: 1,536
    pub embedding: Vec<f64>,

    /// The object type
    pub object: EmbeddingKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingKind {
    Embedding,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbedding {
    /// The input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model, cannot be an empty string, and any array must be 2048 dimensions or less.
    pub input: String,

    /// The id of the model to use.
    pub model: EmbeddingModel,

    /// The format to return the embeddings in. Either:
    ///     - **float**
    ///     - **base64**
    pub encoding_format: EncodingFormat,

    /// The number of dimensions the resulting output embeddings should have. Only supported in **text-embedding-3** and later models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u64>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CreateEmbedding {
    pub fn new(input: impl Into<String>, model: EmbeddingModel) -> Self {
        Self {
            input: input.into(),
            model,
            encoding_format: EncodingFormat::Float,
            dimensions: None,
            user: None,
        }
    }

    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self
    }

    pub fn with_model(mut self, model: EmbeddingModel) -> Self {
        self.model = model;
        self
    }

    pub fn with_encoding_format(mut self, encoding_format: EncodingFormat) -> Self {
        self.encoding_format = encoding_format;
        self
    }

    pub fn with_dimensions(mut self, dimensions: u64) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EmbeddingModel {
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
}

impl fmt::Display for EmbeddingModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextEmbedding3Large => write!(f, "text-embedding-3-large"),
            Self::TextEmbedding3Small => write!(f, "text-embedding-3-small"),
            Self::TextEmbeddingAda002 => write!(f, "text-embedding-ada-002"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodingFormat {
    Float,
    Base64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: EmbeddingModel,
    pub usage: EmbeddingUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingData {
    pub index: u64,
    pub object: EmbeddingKind,
    pub embedding: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingUsage {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
