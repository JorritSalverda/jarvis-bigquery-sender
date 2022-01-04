use std::env;
use std::error::Error;

pub struct NatsConfig {
  pub host: String,
  pub subject: String,
  pub queue: String
}

impl NatsConfig {
  pub async fn new(
    host: String,
    subject: String,
    queue: String
  ) -> Result<Self, Box<dyn Error>> {
      println!(
          "NatsConfig::new(host: {}, subject: {}, queue: {})",
          host, subject, queue
      );

      Ok(Self {
          host,
          subject,
          queue
      })
  }

  pub async fn from_env() -> Result<Self, Box<dyn Error>> {
      let host = env::var("NATS_HOST")
          .unwrap_or_else(|_| String::from("jarvis-nats"));
      let subject = env::var("NATS_SUBJECT")
        .unwrap_or_else(|_| String::from("jarvis-measurements"));
      let queue = env::var("NATS_QUEUE")
        .unwrap_or_else(|_| String::from("jarvis-bigquery-sender"));

      Self::new(
          host,
          subject,
          queue
      )
      .await
  }
}