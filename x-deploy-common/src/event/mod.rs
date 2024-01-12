use crate::CommonResult;
use kafka::client::RequiredAcks;
use kafka::consumer::{Consumer, FetchOffset};
use kafka::producer::{Producer, Record};
use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_slice;
use std::time::Duration;

pub mod organization;
pub mod user;

trait ToTopicName {
  fn topic_name() -> String;
}

pub struct CommonEvent {
  kafka_url: Vec<String>,
}

impl CommonEvent {
  pub fn new(kafka_url: Vec<String>) -> Self {
    Self { kafka_url }
  }

  pub fn send<T>(
    &self,
    data: T,
  ) -> CommonResult<()>
  where
    T: ToTopicName + DeserializeOwned + Serialize,
  {
    let topic = T::topic_name();
    let mut producer = Producer::from_hosts(self.kafka_url.clone())
      // Give the brokers one second time to ack the message.
      .with_ack_timeout(Duration::from_secs(1))
      // Require only one broker to ack the message.
      .with_required_acks(RequiredAcks::One)
      // Build the producer with the above settings.
      .create()?;
    let json_string = serde_json::to_string(&data)?;
    debug!("Sending to topic {} with data: {}...", topic, json_string);
    let record = Record::from_value(&topic, json_string);
    Ok(producer.send(&record)?)
  }

  pub fn loop_consumer<T>(
    &self,
    callback: fn(T) -> CommonResult<()>,
  ) -> CommonResult<()>
  where
    T: ToTopicName + DeserializeOwned + Serialize,
  {
    let topic = T::topic_name();
    debug!("Listening to topic: {}", topic);
    let mut consumer = Consumer::from_hosts(self.kafka_url.clone())
      .with_topic(topic)
      .with_fallback_offset(FetchOffset::Earliest)
      .create()?;
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          let json = from_slice::<T>(m.value).unwrap();
          callback(json)?;
        }
        consumer.consume_messageset(ms)?;
      }
      consumer.commit_consumed()?;
    }
  }
}
