use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const OVH_CRED_COLLECTION_NAME: &str = "ovhCredentials";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OvhCredentialsStatus {
  Pending,
  Active,
  Inactive,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OvhCredentials {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "applicationKey")]
  pub application_key: String,

  #[serde(rename = "applicationSecret")]
  pub application_secret: String,

  #[serde(rename = "consumerKey")]
  pub consumer_key: String,

  #[serde(rename = "status")]
  pub status: OvhCredentialsStatus,

  #[serde(rename = "userId")]
  pub user_id: ObjectId,
}

impl OvhCredentials {
  pub fn new(
    application_key: String,
    application_secret: String,
    consumer_key: String,
    user_id: ObjectId,
    status: OvhCredentialsStatus,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      application_key,
      application_secret,
      consumer_key,
      user_id,
      status,
    }
  }
}
