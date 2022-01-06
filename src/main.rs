mod bigquery_client;

use crate::bigquery_client::{BigqueryClient, BigqueryClientConfig};
use jarvis_lib::model::{Measurement};
use jarvis_lib::nats_client::{NatsClientConfig, NatsClient};
use std::time::Duration;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let bigquery_client_config = BigqueryClientConfig::from_env().await?;
  let bigquery_client = BigqueryClient::new(bigquery_client_config);

  bigquery_client.init_table().await?;

  let nats_client_config = NatsClientConfig::from_env().await?;
  let nats_client = NatsClient::new(nats_client_config);

  let sub = nats_client.queue_subscribe()?;

  for msg in sub.timeout_iter(Duration::from_secs(10)) {
    let measurement: Measurement = serde_json::from_slice(&msg.data).unwrap();
    bigquery_client.insert_measurement(&measurement).await?;
  }

  Ok(())
}
