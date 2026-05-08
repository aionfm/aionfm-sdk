# SDK Usage

Configure `AionFmClient` with a base URL, authentication, timeout, and retry policy. High-level methods map directly to the versioned API routes under `/v1`.

Use `ForecastRequestBuilder` for batch options that are not available on the single-entity convenience call, including retrieval toggles and hierarchy specifications for bottom-up reconciliation.

Call `AionFmClient::evaluate` with an `EvaluationRequest` when realized observations become available. The returned `EvaluationReport` includes point accuracy, quantile pinball loss, empirical coverage, interval coverage, and monitoring alerts.
