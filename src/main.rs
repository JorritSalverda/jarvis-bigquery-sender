mod bigquery_client;

use crate::bigquery_client::{BigqueryClient, BigqueryClientConfig};
use jarvis_lib::model::Measurement;
use jarvis_lib::nats_client::{NatsClient, NatsClientConfig};
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let subscribe_timeout = env::var("NATS_QUEUE_TIMEOUT")
        .unwrap_or_else(|_| String::from("15"))
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Value of envvar NATS_QUEUE_TIMEOUT can't be parsed to u64"));

    let bigquery_client_config = BigqueryClientConfig::from_env().await?;
    let bigquery_client = BigqueryClient::new(bigquery_client_config);

    bigquery_client.init_table().await?;

    let nats_client_config = NatsClientConfig::from_env().await?;
    let mut nats_client = NatsClient::new(nats_client_config);

    let sub = nats_client.subscribe()?;

    // stop looping messages on sigterm
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    while !term.load(Ordering::Relaxed) {
        match sub.next_timeout(Duration::from_secs(subscribe_timeout)) {
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // info!(
                //     "Timed out after {}s waiting for message, continuing",
                //     subscribe_timeout
                // );
                continue;
            }
            Err(e) => panic!("Failed getting next message: {}", e),
            Ok(msg) => {
                let measurement: Measurement =
                    serde_json::from_slice(&msg.data).expect("Failed to deserialize measurement");

                info!(
                    "Received message from source {} with id {}",
                    measurement.source, measurement.id
                );

                bigquery_client
                    .insert_measurement(&measurement)
                    .await
                    .expect("Failed to insert measurement into bigquery table");
            }
        }
    }

    Ok(())
}
