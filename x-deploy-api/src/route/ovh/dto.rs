use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(super) struct Auth {
  #[serde(rename = "applicationKey")]
  pub(super) application_key: String,

  #[serde(rename = "applicationSecret")]
  pub(super) application_secret: String,

  #[serde(rename = "consumerKey")]
  pub(super) consumer_key: String,
}
