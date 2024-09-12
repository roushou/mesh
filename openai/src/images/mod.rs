use core::fmt;
use reqwest::{Client as ReqwestClient, Method, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::error::Error;

pub struct ImageClient {
    base_url: Url,
    http_client: ReqwestClient,
}

impl ImageClient {
    pub fn new(base_url: Url, http_client: ReqwestClient) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    /// Creates an image given a prompt.
    pub async fn create_image(&self, payload: CreateImage) -> Result<CreateImageResponse, Error> {
        let image = self
            .request(Method::POST, "images/generations")?
            .json(&payload)
            .send()
            .await?
            .json::<CreateImageResponse>()
            .await?;
        Ok(image)
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
pub struct CreateImage {
    /// A text description of the desired image(s). The maximum length is 1000 characters for **dall-e-2** and 4000 characters for **dall-e-3**.
    pub prompt: String,

    /// The model to use for image generation.
    pub model: ImageModel,

    /// The number of images to generate. Must be between 1 and 10. For **dall-e-3**, only n=1 is supported.
    pub n: u8,

    /// The quality of the image that will be generated. **hd** creates images with finer details and greater consistency across the image. This param is only supported for **dall-e-3**.
    ///
    /// Defaults to **standard**.
    pub quality: ImageQuality,

    /// The format in which the generated images are returned. Must be one of **url** or **b64_json**.
    pub response_format: ImageResponseFormat,

    /// The size of the generated images.
    ///
    /// For **dall-e-2**:
    ///     - **256x256**
    ///     - **512x512**
    ///     - **1024x1024**
    ///
    /// For **dall-e-3**:
    ///     - **1024x1024**
    ///     - **1792x1024**
    ///     - **1024x1792**
    pub size: ImageSize,

    /// The style of the generated images. Must be one of **vivid** or **natural**. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images.
    ///
    /// Note: This param is only supported for **dall-e-3**
    pub style: ImageStyle,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl CreateImage {
    pub fn new(prompt: impl Into<String>, model: ImageModel) -> Self {
        Self {
            prompt: prompt.into(),
            model,
            n: 1,
            quality: ImageQuality::default(),
            response_format: ImageResponseFormat::default(),
            size: ImageSize::default(),
            style: ImageStyle::default(),
            user: None,
        }
    }

    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = prompt.into();
        self
    }

    pub fn with_model(mut self, model: ImageModel) -> Self {
        self.model = model;
        self
    }

    pub fn with_n(mut self, n: u8) -> Self {
        self.n = if n > 10 { 10 } else { n };
        self
    }

    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.quality = quality;
        self
    }

    pub fn with_response_format(mut self, response_format: ImageResponseFormat) -> Self {
        self.response_format = response_format;
        self
    }

    pub fn with_style(mut self, style: ImageStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageModel {
    #[serde(rename = "dall-e-2")]
    DallE2,
    #[serde(rename = "dall-e-3")]
    DallE3,
}

impl fmt::Display for ImageModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DallE2 => write!(f, "dall-e-2"),
            Self::DallE3 => write!(f, "dall-e-3"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageQuality {
    HD,
    Standard,
}

impl Default for ImageQuality {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImageResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageResponseFormat {
    Url,
    Base64Json,
}

impl Default for ImageResponseFormat {
    fn default() -> Self {
        Self::Url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1792x1024")]
    S1792x1024,
    #[serde(rename = "1024x1792")]
    S1024x1792,
}

impl Default for ImageSize {
    fn default() -> Self {
        Self::S1024x1024
    }
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::S256x256 => write!(f, "256x256"),
            Self::S512x512 => write!(f, "512x512"),
            Self::S1024x1024 => write!(f, "1024x1024"),
            Self::S1792x1024 => write!(f, "1792x1024"),
            Self::S1024x1792 => write!(f, "1024x1792"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageStyle {
    Vivid,
    Natural,
}

impl Default for ImageStyle {
    fn default() -> Self {
        Self::Vivid
    }
}
