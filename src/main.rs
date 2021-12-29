use jarvis_lib::bigquery_client::{BigqueryClient, BigqueryClientConfig};
use jarvis_lib::model::{Measurement};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let bigquery_client_config = BigqueryClientConfig::from_env().await?;
  let bigquery_client = BigqueryClient::new(bigquery_client_config);

  let nc = nats::connect("jarvis-nats")?;
  let sub = nc.subscribe("jarvis-measurements")?;

  for msg in sub.messages() {
    let measurement: Measurement = serde_json::from_slice(&msg.data).unwrap();
    bigquery_client.insert_measurement(&measurement).await?;
  }

  Ok(())
}
