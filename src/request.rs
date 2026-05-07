use crate::{
    AionFmError, AionFmResult, BatchForecastRequest, ForecastConstraints, ForecastEntity,
    ForecastOptions, ForecastRequest, InterpretationRequest, QuantileLevel, RequestOptions,
    ScenarioRequest,
};

/// Builder for batch forecast requests.
#[derive(Clone, Debug, Default)]
pub struct ForecastRequestBuilder {
    entities: Vec<ForecastEntity>,
    options: ForecastOptions,
}

/// Builder for scenario generation requests.
#[derive(Clone, Debug, Default)]
pub struct ScenarioRequestBuilder {
    forecast: ForecastRequestBuilder,
    scenario_type: Option<String>,
    forced_regimes: Vec<String>,
}

impl ScenarioRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn forecast(mut self, forecast: ForecastRequestBuilder) -> Self {
        self.forecast = forecast;
        self
    }

    pub fn scenario_type(mut self, scenario_type: impl Into<String>) -> Self {
        self.scenario_type = Some(scenario_type.into());
        self
    }

    pub fn forced_regime(mut self, regime: impl Into<String>) -> Self {
        self.forced_regimes.push(regime.into());
        self
    }

    pub fn build(self) -> AionFmResult<ScenarioRequest> {
        Ok(ScenarioRequest {
            forecast: self.forecast.build()?,
            scenario_type: self.scenario_type,
            forced_regimes: self.forced_regimes,
        })
    }
}

/// Builder for interpretation requests.
#[derive(Clone, Debug, Default)]
pub struct InterpretationRequestBuilder {
    forecast: ForecastRequestBuilder,
    include_change_points: bool,
    include_attention_summary: bool,
}

impl InterpretationRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn forecast(mut self, forecast: ForecastRequestBuilder) -> Self {
        self.forecast = forecast;
        self
    }

    pub fn include_change_points(mut self, enabled: bool) -> Self {
        self.include_change_points = enabled;
        self
    }

    pub fn include_attention_summary(mut self, enabled: bool) -> Self {
        self.include_attention_summary = enabled;
        self
    }

    pub fn build(self) -> AionFmResult<InterpretationRequest> {
        Ok(InterpretationRequest {
            forecast: self.forecast.return_regimes(true).build()?,
            include_change_points: self.include_change_points,
            include_attention_summary: self.include_attention_summary,
        })
    }
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

    pub fn constraints(mut self, constraints: ForecastConstraints) -> Self {
        self.options.constraints = constraints;
        self.options.enforce_constraints = true;
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
                constraints: self.options.constraints,
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

    #[test]
    fn scenario_builder_sets_type() {
        let entity = ForecastEntity {
            entity_id: "e".into(),
            target: "value".into(),
            historical_values: vec![1.0, 2.0],
            frequency: Default::default(),
            covariates: vec![],
            metadata: Default::default(),
        };
        let request = ScenarioRequestBuilder::new()
            .forecast(
                ForecastRequestBuilder::new()
                    .entity(entity)
                    .horizon(2)
                    .scenarios(3),
            )
            .scenario_type("stress")
            .build()
            .unwrap();
        assert_eq!(request.scenario_type.as_deref(), Some("stress"));
    }
}
