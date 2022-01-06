mod bigquery_client;

use crate::bigquery_client::{BigqueryClient, BigqueryClientConfig};
use jarvis_lib::model::{Measurement};
use jarvis_lib::nats_client::{NatsClientConfig, NatsClient};
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let term = Arc::new(AtomicBool::new(false));
  signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

  let bigquery_client_config = BigqueryClientConfig::from_env().await?;
  let bigquery_client = BigqueryClient::new(bigquery_client_config);

  bigquery_client.init_table().await?;

  let nats_client_config = NatsClientConfig::from_env().await?;
  let nats_client = NatsClient::new(nats_client_config);

  let sub = nats_client.queue_subscribe()?;

  // stop loop on sigterm
  while !term.load(Ordering::Relaxed) {
    match sub.next_timeout(Duration::from_secs(10)) {
      Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
      Err(e) => panic!("Failed getting next message: {}",e),
      Ok(msg) => {
        let measurement: Measurement = serde_json::from_slice(&msg.data).unwrap();
        bigquery_client.insert_measurement(&measurement).await?;
      }
    }
  }

  Ok(())
}
