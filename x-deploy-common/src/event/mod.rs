use crate::CommonResult;
use kafka::client::RequiredAcks;
use kafka::consumer::{Consumer, FetchOffset};
use kafka::producer::{Producer, Record};
use log::info;
use serde::de::DeserializeOwned;
use serde_json::from_slice;
use std::time::Duration;

pub mod organization;
pub mod project;
pub mod user;

pub(crate) fn send_event<T>(
  kafka_url: Vec<String>,
  topic: String,
  json: T,
) -> CommonResult<()>
where
  T: serde::Serialize,
{
  let mut producer = Producer::from_hosts(kafka_url)
    // Give the brokers one second time to ack the message.
    .with_ack_timeout(Duration::from_secs(1))
    // Require only one broker to ack the message.
    .with_required_acks(RequiredAcks::One)
    // Build the producer with the above settings.
    .create()?;
  let json_string = serde_json::to_string(&json)?;
  let record = Record::from_value(&topic, json_string);
  Ok(producer.send(&record)?)
}

pub(crate) fn listen_event<T>(
  kafka_url: Vec<String>,
  topic: String,
  f: fn(T) -> CommonResult<()>,
) -> CommonResult<()>
where
  T: DeserializeOwned,
{
  info!("Listening to topic: {}", topic);
  let mut consumer = Consumer::from_hosts(kafka_url)
    .with_topic(topic)
    .with_fallback_offset(FetchOffset::Earliest)
    .create()?;
  loop {
    for ms in consumer.poll().unwrap().iter() {
      for m in ms.messages() {
        let json = from_slice::<T>(m.value).unwrap();
        f(json)?;
      }
      consumer.consume_messageset(ms)?;
    }
    consumer.commit_consumed()?;
  }
}
