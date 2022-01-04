mod nats_config;

use jarvis_lib::bigquery_client::{BigqueryClient, BigqueryClientConfig};
use jarvis_lib::model::{Measurement};
use nats_config::NatsConfig;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let bigquery_client_config = BigqueryClientConfig::from_env().await?;
  let bigquery_client = BigqueryClient::new(bigquery_client_config);

  let nats_config = NatsConfig::from_env().await?;

  let nc = nats::connect(&nats_config.host)?;
  let sub = nc.queue_subscribe(&nats_config.subject, &nats_config.queue)?;

  for msg in sub.messages() {
    let measurement: Measurement = serde_json::from_slice(&msg.data).unwrap();
    bigquery_client.insert_measurement(&measurement).await?;
  }

  Ok(())
}
