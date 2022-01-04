use std::env;
use std::error::Error;

pub struct NatsConfig {
  pub host: String,
  pub topic: String
}

impl NatsConfig {
  pub async fn new(
    host: String,
    topic: String
  ) -> Result<Self, Box<dyn Error>> {
      println!(
          "NatsConfig::new(host: {}, topic: {})",
          host, topic
      );

      Ok(Self {
          host,
          topic,
      })
  }

  pub async fn from_env() -> Result<Self, Box<dyn Error>> {
      let host = env::var("NATS_HOST")
          .unwrap_or_else(|_| String::from("jarvis-nats"));
      let topic = env::var("NATS_TOPIC")
        .unwrap_or_else(|_| String::from("jarvis-measurements"));

      Self::new(
          host,
          topic
      )
      .await
  }
}