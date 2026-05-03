//! Re-exported public schema types shared with the API and core serving layer.

pub use aionfm_utils::{
    AdaptationRequest, AdaptationStatus, BatchForecastRequest, CovariateSeries, EntityForecast,
    EntityMetadata, ForecastEntity, ForecastOptions, ForecastRequest, ForecastResponse, Frequency,
    InterpretationRequest, ModelDescriptor, PredictionInterval, QuantileLevel, RequestOptions,
    ScenarioRequest, ServiceStatus,
};
