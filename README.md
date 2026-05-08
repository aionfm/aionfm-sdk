# AionFM SDK

Typed async Rust SDK for the AionFM HTTP API described in `../aionfm-spec`.

The SDK wraps forecast, scenario, interpretation, evaluation, model-management, status, and adaptation endpoints while reusing the shared schema from `aionfm-utils`.

## Example

```rust,no_run
use aionfm_sdk::{AionFmClient, ForecastEntity, ForecastOptions};

#[tokio::main]
async fn main() -> Result<(), aionfm_sdk::AionFmError> {
    let client = AionFmClient::new("http://127.0.0.1:8080", "dev-key")?;
    let response = client
        .forecast(
            ForecastEntity {
                entity_id: "store_42".into(),
                target: "demand".into(),
                historical_values: vec![123.0, 125.4, 126.2],
                frequency: Default::default(),
                covariates: vec![],
                metadata: Default::default(),
            },
            ForecastOptions {
                horizon: 7,
                return_regimes: true,
                ..Default::default()
            },
        )
        .await?;
    println!("{:?}", response.results.first());
    Ok(())
}
```

## Commands

```sh
cargo fmt
cargo check
cargo test
```
