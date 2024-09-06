use core::fmt;
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub struct ModerationClient {
    base_url: Url,
    http_client: ReqwestClient,
}

impl ModerationClient {
    pub fn new(base_url: Url, http_client: ReqwestClient) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    pub async fn create_moderation(&self, payload: CreateModeration) -> Result<Moderation, Error> {
        let model = self
            .request(Method::POST, "moderations")?
            .json(&payload)
            .send()
            .await?
            .json::<Moderation>()
            .await?;
        Ok(model)
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
pub struct Moderation {
    /// The unique identifier for the moderation request.
    pub id: String,

    /// The model used to generate the moderation results.
    ///
    /// Note that it corresponds to the exact model version.
    /// For example, the **stable** model may use **text-moderation-007** under-the-hood.
    pub model: String,

    /// List of moderation results.
    pub results: Vec<ModerationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateModeration {
    /// The input text to classify.
    pub input: String,

    /// The content moderation model to use to classify the input. Either:
    ///     - **text-moderation-stable**
    ///     - **text-moderation-latest**
    pub model: ModerationModel,
}

impl CreateModeration {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            model: ModerationModel::Stable,
        }
    }

    pub fn with_input(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self
    }

    pub fn with_model(mut self, model: ModerationModel) -> Self {
        self.model = model;
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModerationModel {
    #[serde(rename = "text-moderation-stable")]
    Stable,
    #[serde(rename = "text-moderation-latest")]
    Latest,
}

impl fmt::Display for ModerationModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stable => write!(f, "text-moderation-stable"),
            Self::Latest => write!(f, "text-moderation-latest"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationResult {
    /// Whether the content has been flagged in any of the moderation categories.
    pub flagged: bool,

    /// List of the moderation categories, and whether they are flagged or not.
    pub categories: ModerationCategories,

    /// List of the moderation categories with their scores as predicted by the model.
    pub category_scores: ModerationCategoryScores,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationCategories {
    pub sexual: bool,
    pub hate: bool,
    pub harassment: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    pub violence: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationCategoryScores {
    pub sexual: f64,
    pub hate: f64,
    pub harassment: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,
    pub violence: f64,
}
