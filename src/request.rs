use crate::{
    AionFmError, AionFmResult, BatchForecastRequest, ForecastEntity, ForecastOptions,
    ForecastRequest, QuantileLevel, RequestOptions,
};

/// Builder for batch forecast requests.
#[derive(Clone, Debug, Default)]
pub struct ForecastRequestBuilder {
    entities: Vec<ForecastEntity>,
    options: ForecastOptions,
}

impl ForecastRequestBuilder {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            options: ForecastOptions::default(),
        }
    }

    pub fn entity(mut self, entity: ForecastEntity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn entities(mut self, entities: impl IntoIterator<Item = ForecastEntity>) -> Self {
        self.entities.extend(entities);
        self
    }

    pub fn horizon(mut self, horizon: usize) -> Self {
        self.options.horizon = horizon;
        self
    }

    pub fn quantiles(mut self, quantiles: impl IntoIterator<Item = QuantileLevel>) -> Self {
        self.options.quantiles = quantiles.into_iter().collect();
        self
    }

    pub fn scenarios(mut self, count: usize) -> Self {
        self.options.scenario_count = Some(count);
        self.options.return_scenarios = true;
        self
    }

    pub fn return_regimes(mut self, enabled: bool) -> Self {
        self.options.return_regimes = enabled;
        self
    }

    pub fn enforce_constraints(mut self, enabled: bool) -> Self {
        self.options.enforce_constraints = enabled;
        self
    }

    pub fn use_retrieval(mut self, enabled: bool) -> Self {
        self.options.use_retrieval = enabled;
        self
    }

    pub fn build(self) -> AionFmResult<BatchForecastRequest> {
        let Some(first) = self.entities.first() else {
            return Err(AionFmError::Unexpected(
                "forecast request requires at least one entity".into(),
            ));
        };
        Ok(BatchForecastRequest {
            request_id: ForecastRequest::new(first.clone(), self.options.clone()).request_id,
            entities: self.entities,
            horizon: self.options.horizon,
            quantiles: self.options.quantiles,
            scenario_count: self.options.scenario_count,
            options: RequestOptions {
                return_regimes: self.options.return_regimes,
                return_scenarios: self.options.return_scenarios,
                enforce_constraints: self.options.enforce_constraints,
                use_retrieval: self.options.use_retrieval,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_rejects_empty_batch() {
        assert!(ForecastRequestBuilder::new().build().is_err());
    }
}
