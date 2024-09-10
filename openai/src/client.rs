use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Url,
};

use crate::{
    chat::ChatClient, config::Config, embeddings::EmbeddingClient, error::Error,
    models::ModelClient, moderations::ModerationClient,
};

pub struct Client {
    api_key: String,
    base_url: Url,
    pub chat: ChatClient,
    pub model: ModelClient,
    pub moderation: ModerationClient,
    pub embedding: EmbeddingClient,
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
            chat: ChatClient::new(base_url.clone(), http_client.clone()),
            model: ModelClient::new(base_url.clone(), http_client.clone()),
            moderation: ModerationClient::new(base_url.clone(), http_client.clone()),
            embedding: EmbeddingClient::new(base_url, http_client),
        })
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }
}
