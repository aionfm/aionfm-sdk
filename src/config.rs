use crate::{AionFmError, RetryPolicy};
use aionfm_utils::{PrivacyMode, RequestContext};
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
    pub context: RequestContext,
    pub timeout: Duration,
    pub retry: RetryPolicy,
}

impl AionFmConfig {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self, AionFmError> {
        Ok(Self {
            base_url: Url::parse(base_url.as_ref())?,
            auth: AuthConfig::None,
            context: RequestContext::default(),
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

    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.context.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn with_actor_id(mut self, actor_id: impl Into<String>) -> Self {
        self.context.actor_id = Some(actor_id.into());
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.context.trace_id = Some(trace_id.into());
        self
    }

    pub fn with_purpose(mut self, purpose: impl Into<String>) -> Self {
        self.context.purpose = Some(purpose.into());
        self
    }

    pub fn with_privacy_mode(mut self, privacy_mode: PrivacyMode) -> Self {
        self.context.privacy_mode = privacy_mode;
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
        if let Some(tenant_id) = &self.context.tenant_id {
            headers.insert("x-aionfm-tenant-id", HeaderValue::from_str(tenant_id)?);
        }
        if let Some(actor_id) = &self.context.actor_id {
            headers.insert("x-aionfm-actor-id", HeaderValue::from_str(actor_id)?);
        }
        if let Some(trace_id) = &self.context.trace_id {
            headers.insert("x-request-id", HeaderValue::from_str(trace_id)?);
        }
        if let Some(purpose) = &self.context.purpose {
            headers.insert("x-aionfm-purpose", HeaderValue::from_str(purpose)?);
        }
        if self.context.privacy_mode != PrivacyMode::Standard {
            headers.insert(
                "x-aionfm-privacy-mode",
                HeaderValue::from_str(&self.context.privacy_mode.to_string())?,
            );
        }
        Ok(headers)
    }

    pub fn endpoint(&self, path: &str) -> Result<Url, AionFmError> {
        Ok(self.base_url.join(path.trim_start_matches('/'))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attaches_governance_headers() {
        let headers = AionFmConfig::new("https://api.example.com")
            .unwrap()
            .with_tenant_id("tenant_a")
            .with_actor_id("analyst_1")
            .with_privacy_mode(PrivacyMode::TenantIsolated)
            .headers()
            .unwrap();
        assert_eq!(headers["x-aionfm-tenant-id"], "tenant_a");
        assert_eq!(headers["x-aionfm-actor-id"], "analyst_1");
        assert_eq!(headers["x-aionfm-privacy-mode"], "tenant_isolated");
    }
}
