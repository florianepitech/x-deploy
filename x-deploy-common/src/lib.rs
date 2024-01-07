pub mod cache;
pub mod db;
pub mod event;

pub type CommonResult<T> = Result<T, CommonError>;

#[derive(Debug)]
pub enum CommonError {
  DbError(mongodb::error::Error),
  BsonDeserializeError(bson::de::Error),
  BsonSerializeError(bson::ser::Error),
  SerdeJsonError(serde_json::Error),
  KafkaError(kafka::error::Error),
  RedisError(redis::RedisError),
}

impl From<mongodb::error::Error> for CommonError {
  fn from(err: mongodb::error::Error) -> Self {
    Self::DbError(err)
  }
}

impl From<bson::de::Error> for CommonError {
  fn from(err: bson::de::Error) -> Self {
    Self::BsonDeserializeError(err)
  }
}

impl From<bson::ser::Error> for CommonError {
  fn from(err: bson::ser::Error) -> Self {
    Self::BsonSerializeError(err)
  }
}

impl From<serde_json::Error> for CommonError {
  fn from(err: serde_json::Error) -> Self {
    Self::SerdeJsonError(err)
  }
}

impl From<kafka::error::Error> for CommonError {
  fn from(err: kafka::error::Error) -> Self {
    Self::KafkaError(err)
  }
}

impl From<redis::RedisError> for CommonError {
  fn from(err: redis::RedisError) -> Self {
    Self::RedisError(err)
  }
}
