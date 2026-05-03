use aionfm_sdk::{AionFmClient, ForecastEntity, ForecastOptions};

#[tokio::main]
async fn main() -> Result<(), aionfm_sdk::AionFmError> {
    let client = AionFmClient::new("http://127.0.0.1:8080", "dev-key")?;
    let entity = ForecastEntity {
        entity_id: "store_42".into(),
        target: "demand".into(),
        historical_values: vec![123.0, 125.4, 126.2],
        frequency: Default::default(),
        covariates: vec![],
        metadata: Default::default(),
    };
    let response = client
        .forecast(
            entity,
            ForecastOptions {
                horizon: 7,
                return_regimes: true,
                ..Default::default()
            },
        )
        .await?;
    println!("{:#?}", response);
    Ok(())
}
