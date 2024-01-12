use crate::event::ToTopicName;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const USER_REGISTERED_TOPIC: &str = "user.registered";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisteredEvent {
  pub id: ObjectId,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
}

impl ToTopicName for UserRegisteredEvent {
  fn topic_name() -> String {
    USER_REGISTERED_TOPIC.to_string()
  }
}

pub const USER_MAGIC_LINK_TOPIC: &str = "user.magic_link";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserMagicLinkEvent {
  pub id: ObjectId,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
  pub jwt: String,
}

impl ToTopicName for UserMagicLinkEvent {
  fn topic_name() -> String {
    USER_MAGIC_LINK_TOPIC.to_string()
  }
}

pub const USER_FORGOT_PASSWORD_TOPIC: &str = "user.forgot_password";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserForgotPasswordEvent {
  pub id: ObjectId,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
  pub token: String,
}

impl ToTopicName for UserForgotPasswordEvent {
  fn topic_name() -> String {
    USER_FORGOT_PASSWORD_TOPIC.to_string()
  }
}

pub const USER_PASSWORD_RESET_TOPIC: &str = "user.password_reset";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPasswordResetEvent {
  pub id: ObjectId,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
}

impl ToTopicName for UserPasswordResetEvent {
  fn topic_name() -> String {
    USER_PASSWORD_RESET_TOPIC.to_string()
  }
}
