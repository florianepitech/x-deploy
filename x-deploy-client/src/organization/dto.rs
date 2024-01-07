use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrganizationInfo {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "website")]
  pub website: String,

  #[serde(rename = "contactEmail")]
  pub contact_email: String,
}
