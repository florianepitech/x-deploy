use crate::CONFIG;
use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};
use serde_json::Value;
use std::time::Duration;

pub(crate) mod organization;
pub(crate) mod project;
pub(crate) mod user;

pub(super) fn send_event(
  topic: String,
  json: Value,
) -> Result<(), kafka::error::Error> {
  debug!("Send event: {} {}", topic, json);
  let kafka_url = CONFIG.kafka_url.clone();
  let mut producer = Producer::from_hosts(kafka_url)
    // Give the brokers one second time to ack the message.
    .with_ack_timeout(Duration::from_secs(1))
    // Require only one broker to ack the message.
    .with_required_acks(RequiredAcks::One)
    // Build the producer with the above settings.
    .create()?;
  let json_string = json.to_string();
  let record = Record::from_value(&topic, json_string);
  Ok(producer.send(&record)?)
}
