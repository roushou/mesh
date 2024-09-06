use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Method, RequestBuilder, Url,
};

use crate::{
    chat::{
        message::{ChatCompletion, CreateChatCompletion},
        ChatClient,
    },
    config::Config,
    error::{ApiErrorResponse, Error},
    models::ModelClient,
    moderations::ModerationClient,
};

pub struct Client {
    api_key: String,
    base_url: Url,
    http_client: reqwest::Client,
    pub chat: ChatClient,
    pub model: ModelClient,
    pub moderation: ModerationClient,
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
            http_client: http_client.clone(),
            chat: ChatClient::new(base_url.clone(), http_client.clone()),
            model: ModelClient::new(base_url.clone(), http_client.clone()),
            moderation: ModerationClient::new(base_url, http_client),
        })
    }

    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let url = self
            .base_url
            .join(path)
            .map_err(|err| Error::UrlParse(err.to_string()))?;
        Ok(self.http_client.request(method, url))
    }

    pub async fn create_message(
        &self,
        payload: CreateChatCompletion,
    ) -> Result<ChatCompletion, Error> {
        let response = self
            .request(Method::POST, "messages")?
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            match serde_json::from_str::<ApiErrorResponse>(&error) {
                Ok(api_error) => return Err(Error::Api(api_error)),
                Err(err) => return Err(Error::JsonDeserialize(err)),
            }
        }

        response.json::<ChatCompletion>().await.map_err(Error::from)
    }

    // pub async fn stream_message(
    //     &self,
    //     request: MessageRequest,
    // ) -> Result<impl Stream<Item = Result<StreamEvent, Error>>, Error> {
    //     let response = self
    //         .request(Method::POST, "messages")?
    //         .header(ACCEPT, "text/event-stream")
    //         .json(&request)
    //         .send()
    //         .await?;
    //
    //     if !response.status().is_success() {
    //         let error = response.text().await?;
    //         match serde_json::from_str::<ApiErrorResponse>(&error) {
    //             Ok(api_error) => return Err(Error::Api(api_error)),
    //             Err(err) => return Err(Error::JsonDeserialize(err)),
    //         }
    //     }
    //
    //     Ok(response.bytes_stream().flat_map(move |chunk| match chunk {
    //         Ok(bytes) => {
    //             let events = Self::parse_stream_chunk(&bytes);
    //             stream::iter(events)
    //         }
    //         Err(err) => stream::iter(vec![Err(Error::from(err))]),
    //     }))
    // }

    // fn parse_stream_chunk(bytes: &[u8]) -> Vec<Result<StreamEvent, Error>> {
    //     let chunk_str = match std::str::from_utf8(bytes).map_err(Error::Utf8Error) {
    //         Ok(chunk_str) => chunk_str,
    //         Err(err) => return vec![Err(err)],
    //     };
    //     chunk_str
    //         .split("\n\n")
    //         .filter(|event| !event.trim().is_empty())
    //         .map(|event| {
    //             event
    //                 .lines()
    //                 .find(|line| line.starts_with("data: "))
    //                 .and_then(|line| line.strip_prefix("data: "))
    //                 .ok_or(Error::InvalidStreamEvent)
    //                 .and_then(|content| {
    //                     StreamEvent::from_str(content).map_err(|_| Error::InvalidStreamEvent)
    //                 })
    //         })
    //         .collect()
    // }
}
