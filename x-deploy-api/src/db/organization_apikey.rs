use rocket::serde::{Deserialize, Serialize};

pub(crate) const ORGANIZATION_APIKEY_COLLECTION_NAME: &str = "organizations";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationApiKey {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "organizationId")]
  pub organization_id: String,

  #[serde(rename = "key")]
  pub key: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "expiresAt")]
  pub expires_at: i64,
}
