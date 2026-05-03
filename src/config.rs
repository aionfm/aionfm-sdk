use crate::{AionFmError, RetryPolicy};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::time::Duration;
use url::Url;

/// Authentication configuration for SDK requests.
#[derive(Clone, Debug)]
pub enum AuthConfig {
    None,
    ApiKey(String),
    BearerToken(String),
}

/// SDK client configuration.
#[derive(Clone, Debug)]
pub struct AionFmConfig {
    pub base_url: Url,
    pub auth: AuthConfig,
    pub timeout: Duration,
    pub retry: RetryPolicy,
}

impl AionFmConfig {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self, AionFmError> {
        Ok(Self {
            base_url: Url::parse(base_url.as_ref())?,
            auth: AuthConfig::None,
            timeout: Duration::from_secs(30),
            retry: RetryPolicy::default(),
        })
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.auth = AuthConfig::ApiKey(api_key.into());
        self
    }

    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        self.auth = AuthConfig::BearerToken(token.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn headers(&self) -> Result<HeaderMap, AionFmError> {
        let mut headers = HeaderMap::new();
        match &self.auth {
            AuthConfig::None => {}
            AuthConfig::ApiKey(key) => {
                headers.insert("x-api-key", HeaderValue::from_str(key)?);
            }
            AuthConfig::BearerToken(token) => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {token}"))?,
                );
            }
        }
        Ok(headers)
    }

    pub fn endpoint(&self, path: &str) -> Result<Url, AionFmError> {
        Ok(self.base_url.join(path.trim_start_matches('/'))?)
    }
}
