use crate::event::{listen_event, send_event};
use crate::CommonResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct UserEvent {}

pub const USER_REGISTERED_TOPIC: &str = "user.registered";

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
  pub id: bson::oid::ObjectId,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
}

impl UserEvent {
  pub fn send_registered(
    kafka_url: Vec<String>,
    body: UserRegisteredEvent,
  ) -> CommonResult<()> {
    Ok(send_event(
      kafka_url,
      USER_REGISTERED_TOPIC.to_string(),
      body,
    )?)
  }

  pub async fn listen_registered(
    kafka_url: Vec<String>,
    f: fn(UserRegisteredEvent) -> CommonResult<()>,
  ) -> CommonResult<()> {
    Ok(listen_event(
      kafka_url,
      USER_REGISTERED_TOPIC.to_string(),
      f,
    )?)
  }
}

#[deprecated]
pub fn send_user_registered_event(
  kafka_url: Vec<String>,
  object_id: bson::oid::ObjectId,
  firstname: String,
  lastname: String,
  email: String,
) -> CommonResult<()> {
  let json: Value = json!({
    "id": object_id.to_string(),
    "firstname": firstname,
    "lastname": lastname,
    "email": email,
  });
  send_event(kafka_url, USER_REGISTERED_TOPIC.to_string(), json)
}

pub const USER_MAGIC_LINK_TOPIC: &str = "user.magic_link";

pub fn send_magic_link_event(
  kafka_url: Vec<String>,
  object_id: bson::oid::ObjectId,
  email: String,
  jwt: String,
) -> CommonResult<()> {
  let json: Value = json!({
    "id": object_id.to_string(),
    "email": email,
    "jwt": jwt,
  });
  send_event(kafka_url, USER_MAGIC_LINK_TOPIC.to_string(), json)
}

pub const USER_FORGOT_PASSWORD_TOPIC: &str = "user.forgot_password";

pub fn send_forgot_password_event(
  kafka_url: Vec<String>,
  object_id: bson::oid::ObjectId,
  email: String,
  firstname: String,
  lastname: String,
  token: String,
) -> CommonResult<()> {
  let json: Value = json!({
    "id": object_id.to_string(),
    "firstname": firstname,
    "lastname": lastname,
    "email": email,
    "token": token,
  });
  send_event(kafka_url, USER_FORGOT_PASSWORD_TOPIC.to_string(), json)
}

pub const USER_PASSWORD_RESET_TOPIC: &str = "user.password_reset";

pub fn send_password_reset_event(
  kafka_url: Vec<String>,
  object_id: bson::oid::ObjectId,
  firstname: String,
  lastname: String,
  email: String,
) -> CommonResult<()> {
  let json: Value = json!({
    "id": object_id.to_string(),
    "firstname": firstname,
    "lastname": lastname,
    "email": email,
  });
  send_event(kafka_url, USER_PASSWORD_RESET_TOPIC.to_string(), json)
}
