use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountCurrentCredential {
  #[serde(rename = "allowedIPs")]
  pub allowed_ips: Option<Vec<String>>,

  #[serde(rename = "applicationId")]
  pub application_id: u64,

  #[serde(rename = "creation")]
  pub creation: String,

  #[serde(rename = "credentialId")]
  pub credential_id: u64,

  #[serde(rename = "expiration")]
  pub expiration: String,

  #[serde(rename = "lastUse")]
  pub last_use: String,

  #[serde(rename = "ovhSupport")]
  pub ovh_support: bool,

  #[serde(rename = "rules")]
  pub rules: Vec<Rule>,

  #[serde(rename = "status")]
  pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
  #[serde(rename = "method")]
  pub method: String,

  #[serde(rename = "path")]
  pub path: String,
}
