use crate::{
    AdaptationRequest, AdaptationStatus, AionFmConfig, AionFmError, AionFmResult,
    BatchForecastRequest, ForecastEntity, ForecastOptions, ForecastRequest, ForecastResponse,
    InterpretationRequest, ModelDescriptor, RequestOptions, ScenarioRequest, ServiceStatus,
};
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::instrument;

/// Typed asynchronous AionFM API client.
#[derive(Clone)]
pub struct AionFmClient {
    config: AionFmConfig,
    http: reqwest::Client,
}

impl AionFmClient {
    pub fn new(base_url: impl AsRef<str>, api_key: impl Into<String>) -> AionFmResult<Self> {
        Self::from_config(AionFmConfig::new(base_url)?.with_api_key(api_key))
    }

    pub fn unauthenticated(base_url: impl AsRef<str>) -> AionFmResult<Self> {
        Self::from_config(AionFmConfig::new(base_url)?)
    }

    pub fn from_config(config: AionFmConfig) -> AionFmResult<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(config.headers()?)
            .build()?;
        Ok(Self { config, http })
    }

    #[instrument(skip(self, entity))]
    pub async fn forecast(
        &self,
        entity: ForecastEntity,
        options: ForecastOptions,
    ) -> AionFmResult<ForecastResponse> {
        let request = BatchForecastRequest {
            request_id: ForecastRequest::new(entity.clone(), options.clone()).request_id,
            entities: vec![entity],
            horizon: options.horizon,
            quantiles: options.quantiles,
            scenario_count: options.scenario_count,
            options: RequestOptions {
                return_regimes: options.return_regimes,
                return_scenarios: options.return_scenarios,
                enforce_constraints: options.enforce_constraints,
                use_retrieval: options.use_retrieval,
            },
        };
        self.forecast_batch(request).await
    }

    #[instrument(skip(self, request))]
    pub async fn forecast_batch(
        &self,
        request: BatchForecastRequest,
    ) -> AionFmResult<ForecastResponse> {
        self.send_json(Method::POST, "/v1/forecast", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn generate_scenario(
        &self,
        request: ScenarioRequest,
    ) -> AionFmResult<ForecastResponse> {
        self.send_json(Method::POST, "/v1/scenario", Some(&request))
            .await
    }

    #[instrument(skip(self, request))]
    pub async fn interpret(
        &self,
        request: InterpretationRequest,
    ) -> AionFmResult<ForecastResponse> {
        self.send_json(Method::POST, "/v1/interpretation", Some(&request))
            .await
    }

    pub async fn models(&self) -> AionFmResult<Vec<ModelDescriptor>> {
        self.send_json::<(), _>(Method::GET, "/v1/models", None)
            .await
    }

    pub async fn status(&self) -> AionFmResult<ServiceStatus> {
        self.send_json::<(), _>(Method::GET, "/v1/status", None)
            .await
    }

    pub async fn adapt(&self, request: AdaptationRequest) -> AionFmResult<AdaptationStatus> {
        self.send_json(Method::POST, "/v1/adapt", Some(&request))
            .await
    }

    async fn send_json<B, R>(&self, method: Method, path: &str, body: Option<&B>) -> AionFmResult<R>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.config.endpoint(path)?;
        let mut attempt = 0;
        loop {
            let mut request = self.http.request(method.clone(), url.clone());
            if let Some(body) = body {
                request = request.json(body);
            }
            let response = request.send().await;
            match response {
                Ok(response) if response.status().is_success() => {
                    return Ok(response.json::<R>().await?)
                }
                Ok(response)
                    if should_retry(response.status())
                        && attempt < self.config.retry.max_retries =>
                {
                    let backoff = self.config.retry.backoff_for(attempt);
                    attempt += 1;
                    tokio::time::sleep(backoff).await;
                }
                Ok(response) => {
                    let status = response.status().as_u16();
                    let message = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "unable to read error body".into());
                    return Err(AionFmError::Api {
                        status,
                        message: parse_api_error_message(&message),
                    });
                }
                Err(error) if attempt < self.config.retry.max_retries => {
                    let backoff = self.config.retry.backoff_for(attempt);
                    attempt += 1;
                    tracing::debug!(%error, ?backoff, "retrying AionFM request");
                    tokio::time::sleep(backoff).await;
                }
                Err(error) => return Err(error.into()),
            }
        }
    }
}

fn should_retry(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    message: String,
}

fn parse_api_error_message(body: &str) -> String {
    serde_json::from_str::<ApiErrorBody>(body)
        .map(|body| body.message)
        .unwrap_or_else(|_| body.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_v1_endpoint() {
        let client = AionFmClient::unauthenticated("https://api.example.com/base/").unwrap();
        assert_eq!(
            client.config.endpoint("/v1/status").unwrap().as_str(),
            "https://api.example.com/base/v1/status"
        );
    }

    #[test]
    fn parses_standard_api_error_body() {
        let message =
            parse_api_error_message(r#"{"code":400,"error":"Bad Request","message":"invalid"}"#);
        assert_eq!(message, "invalid");
    }
}
