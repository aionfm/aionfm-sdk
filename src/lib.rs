//! Typed async Rust SDK for AionFM.

pub mod client;
pub mod config;
pub mod error;
pub mod models;
pub mod request;
pub mod retry;

pub use client::AionFmClient;
pub use config::{AionFmConfig, AuthConfig};
pub use error::{AionFmError, AionFmResult};
pub use models::*;
pub use request::{ForecastRequestBuilder, InterpretationRequestBuilder, ScenarioRequestBuilder};
pub use retry::RetryPolicy;
