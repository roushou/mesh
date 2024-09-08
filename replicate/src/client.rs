use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Method, RequestBuilder, Response, StatusCode, Url,
};
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::Error};

pub struct Client {
    api_key: String,
    base_url: Url,
    http_client: reqwest::Client,
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
            base_url,
            http_client,
        })
    }

    /// Get the api key.
    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    /// Get the authenticated account.
    pub async fn account(&self) -> Result<Account, Error> {
        let response = self.request(Method::GET, "account")?.send().await?;
        self.handle_response::<Account>(response).await
    }

    /// List collections of models.
    pub async fn collections(&self) -> Result<ListCollections, Error> {
        let response = self.request(Method::GET, "collections")?.send().await?;
        self.handle_response::<ListCollections>(response).await
    }

    /// List collection of models.
    pub async fn collection_models(
        &self,
        collection: String,
    ) -> Result<ListCollectionModels, Error> {
        let path = format!("collections/{}", collection);
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<ListCollectionModels>(response).await
    }

    /// Get information about a deployment by name including the current release.
    pub async fn deployment(&self, owner: String, name: String) -> Result<Deployment, Error> {
        let path = format!("deployments/{}/{}", owner, name);
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<Deployment>(response).await
    }

    /// List deployments associated with the current account, including the latest release configuration for each deployment.
    pub async fn deployments(&self) -> Result<ListDeployments, Error> {
        let response = self.request(Method::GET, "deployments")?.send().await?;
        self.handle_response::<ListDeployments>(response).await
    }

    /// Create a new deployment.
    pub async fn create_deployment(&self, payload: CreateDeployment) -> Result<Deployment, Error> {
        let response = self
            .request(Method::POST, "deployments")?
            .json(&payload)
            .send()
            .await?;
        self.handle_response::<Deployment>(response).await
    }

    /// Update a deployment.
    pub async fn update_deployment(
        &self,
        owner: String,
        payload: UpdateDeployment,
    ) -> Result<Deployment, Error> {
        let path = format!("deployments/{}/{}", owner, payload.name);
        let response = self
            .request(Method::PATCH, path.as_str())?
            .json(&payload)
            .send()
            .await?;
        self.handle_response::<Deployment>(response).await
    }

    /// Delete a deployment.
    ///
    /// Deployment deletion has some restrictions:
    ///     - You can only delete deployments that have been offline and unused for at least 15 minutes.
    pub async fn delete_deployment(&self, owner: String, name: String) -> Result<(), Error> {
        let path = format!("deployments/{}/{}", owner, name);
        let response = self.request(Method::DELETE, path.as_str())?.send().await?;
        self.handle_response(response).await
    }

    /// Get a prediction.
    pub async fn prediction(&self, prediction_id: String) -> Result<Prediction, Error> {
        let path = format!("predictions/{}", prediction_id);
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<Prediction>(response).await
    }

    /// List predictions.
    pub async fn predictions(&self) -> Result<ListPredictions, Error> {
        let response = self.request(Method::GET, "predictions")?.send().await?;
        self.handle_response::<ListPredictions>(response).await
    }

    /// Create a prediction.
    pub async fn create_prediction(&self, payload: CreatePrediction) -> Result<Prediction, Error> {
        let response = self
            .request(Method::POST, "predictions")?
            .json(&payload)
            .send()
            .await?;
        self.handle_response::<Prediction>(response).await
    }

    /// Create a prediction from an official model
    pub async fn create_model_prediction(
        &self,
        payload: CreateModelPrediction,
    ) -> Result<Prediction, Error> {
        let path = format!("models/{}/{}/predictions", payload.owner, payload.name);
        let response = self
            .request(Method::POST, path.as_str())?
            .json(&serde_json::json!({ "input": payload.input }))
            .send()
            .await?;
        self.handle_response::<Prediction>(response).await
    }

    /// Cancel a prediction.
    pub async fn cancel_prediction(&self, prediction_id: String) -> Result<(), Error> {
        let path = format!("predictions/{}/cancel", prediction_id);
        let response = self.request(Method::POST, path.as_str())?.send().await?;
        self.handle_response(response).await
    }

    /// Get a training.
    pub async fn training(&self, training_id: String) -> Result<Training, Error> {
        let path = format!("trainings/{}", training_id);
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<Training>(response).await
    }

    /// List trainings.
    pub async fn trainings(&self) -> Result<ListTrainings, Error> {
        let response = self.request(Method::GET, "trainings")?.send().await?;
        self.handle_response::<ListTrainings>(response).await
    }

    /// Cancel a training.
    pub async fn cancel_training(&self, training_id: String) -> Result<(), Error> {
        let path = format!("trainings/{}/cancel", training_id);
        let response = self.request(Method::POST, path.as_str())?.send().await?;
        self.handle_response(response).await
    }

    /// List available hardware for models.
    pub async fn hardware(&self) -> Result<Vec<Hardware>, Error> {
        let response = self.request(Method::GET, "hardware")?.send().await?;
        self.handle_response::<Vec<Hardware>>(response).await
    }

    /// List public models.
    pub async fn public_models(&self) -> Result<ListPublicModels, Error> {
        let response = self.request(Method::GET, "models")?.send().await?;
        self.handle_response::<ListPublicModels>(response).await
    }

    /// Get model.
    pub async fn model(
        &self,
        owner: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Model, Error> {
        let path = format!("models/{}/{}", owner.into(), name.into());
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<Model>(response).await
    }

    /// List model versions.
    pub async fn model_versions(
        &self,
        owner: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<ListModelVersions, Error> {
        let path = format!("models/{}/{}/versions", owner.into(), name.into());
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<ListModelVersions>(response).await
    }

    /// Get model version.
    pub async fn model_version(
        &self,
        owner: impl Into<String>,
        name: impl Into<String>,
        version_id: impl Into<String>,
    ) -> Result<ModelVersion, Error> {
        let path = format!(
            "models/{}/{}/versions/{}",
            owner.into(),
            name.into(),
            version_id.into()
        );
        let response = self.request(Method::GET, path.as_str())?.send().await?;
        self.handle_response::<ModelVersion>(response).await
    }

    /// Get WebHook default secret
    pub async fn webhook_default_secret(&self) -> Result<WebHookSecret, Error> {
        let response = self
            .request(Method::GET, "webhooks/default/secret")?
            .send()
            .await?;
        self.handle_response::<WebHookSecret>(response).await
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
                StatusCode::BAD_REQUEST => {
                    let error_msg = response.text().await?;
                    Err(Error::BadRequest(error_msg))
                }
                StatusCode::UNAUTHORIZED => {
                    let error_msg = response.text().await?;
                    Err(Error::Unauthorized(error_msg))
                }
                StatusCode::FORBIDDEN => {
                    let error_msg = response.text().await?;
                    Err(Error::Forbidden(error_msg))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    let error_msg = response.text().await?;
                    Err(Error::RateLimited(error_msg))
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    let error_msg = response.text().await?;
                    Err(Error::InternalServerError(error_msg))
                }
                StatusCode::SERVICE_UNAVAILABLE => {
                    let error_msg = response.text().await?;
                    Err(Error::ServiceUnavailable(error_msg))
                }
                status => Err(Error::UnexpectedStatus(status)),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "type")]
    pub kind: AccountKind,
    pub username: String,
    pub name: String,
    pub github_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountKind {
    Organization,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollections {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Collection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCollectionModels {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub models: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPublicModels {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Model>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub owner: String,
    pub visibility: ModelVisibility,
    pub github_url: Option<String>,
    pub paper_url: Option<String>,
    pub license_url: Option<String>,
    pub run_count: u64,
    pub cover_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelVisibility {
    Private,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeployment {
    /// The name of the deployment.
    pub name: String,

    /// The full name of the model that you want to deploy e.g. stability-ai/sdxl.
    pub model: String,

    /// The 64-character string ID of the model version that you want to deploy.
    pub version: String,

    /// The SKU for the hardware used to run the model.
    pub hardware: String,

    /// The maximum number of instances for scaling.
    pub min_instances: u16,

    /// The minimum number of instances for scaling.
    pub max_instances: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeployment {
    /// The name of the deployment.
    pub name: String,

    /// The full name of the model that you want to deploy e.g. stability-ai/sdxl.
    pub model: String,

    /// The 64-character string ID of the model version that you want to deploy.
    pub version: String,

    /// The SKU for the hardware used to run the model.
    pub hardware: String,

    /// The maximum number of instances for scaling.
    pub min_instances: u16,

    /// The minimum number of instances for scaling.
    pub max_instances: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDeployments {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Deployment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub owner: String,
    pub name: String,
    pub current_release: DeploymentRelease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRelease {
    pub number: u64,
    pub model: String,
    pub version: String,
    pub created_at: String,
    pub created_by: Account,
    pub configuration: DeploymentConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    pub hardware: String,
    pub min_instances: u16,
    pub max_instances: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrediction {
    /// The ID of the model version to run.
    pub version: String,

    /// The model's input as a JSON object.
    pub input: serde_json::Value,

    /// An HTTPS URL for receiving a webhook when the prediction has new output.
    ///
    /// The webhook will be a POST request where the request body is the same as the response body of the get prediction operation.
    ///
    /// **Notes**:
    ///     - Retries a few times in case of network problems.
    ///     - It doesn't follow redirects.
    pub webhook: Option<String>,

    /// Events triggering webhook requests.
    ///
    /// **start**: immediately on prediction start
    /// **output**: each time a prediction generates an output (note that predictions can generate multiple outputs)
    /// **logs**: each time log output is generated by a prediction
    /// **completed**: when the prediction reaches a terminal state (succeeded/canceled/failed)
    ///
    /// For example, if you only wanted requests to be sent at the start and end of the prediction, you would provide:
    ///
    /// ```json
    /// {
    ///     "version":
    ///     "5c7d5dc6dd8bf75c1acaa8565735e7986bc5b66206b55cca93cb72c9bf15ccaa",
    ///     "input": {
    ///         "text": "Alice"
    ///     },
    ///     "webhook": "https://example.com/my-webhook",
    ///     "webhook_events_filter": ["start", "completed"]
    /// }
    /// ```
    pub webhook_event_filters: Option<Vec<WebHookEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateModelPrediction {
    /// Model owner
    pub owner: String,

    /// Model name
    pub name: String,

    /// The model's input as a JSON object.
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WebHookEvent {
    Start,
    Output,
    Logs,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPredictions {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Prediction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: String,
    pub model: String,
    pub version: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub source: Option<Source>,
    pub metrics: Option<PredictionMetrics>,
    pub status: PredictionStatus,
    pub urls: PredictionUrls,
    pub logs: Option<String>,
    pub data_removed: Option<bool>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PredictionStatus {
    Starting,
    Processing,
    Succeeded,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionMetrics {
    pub predict_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionUrls {
    pub get: String,
    pub cancel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Web,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTrainings {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Training>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Training {
    pub completed_at: String,
    pub created_at: String,
    pub id: String,
    pub input: serde_json::Value,
    pub metrics: TrainingMetrics,
    pub output: TrainingOutput,
    pub started_at: String,
    pub source: Source,
    pub status: String,
    pub urls: TrainingUrls,
    pub model: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    pub predict_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingOutput {
    pub version: String,
    pub weights: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingUrls {
    pub get: String,
    pub cancel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hardware {
    pub name: String,
    pub sku: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListModelVersions {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<ModelVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVersion {
    pub id: String,
    pub created_at: String,
    pub cog_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebHookSecret {
    pub key: String,
}
