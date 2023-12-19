use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub(crate) struct CreateApiKeyBody {
  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "expiresAt")]
  pub(crate) expires_at: i64,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub(crate) struct ApiKey {
  #[serde(rename = "id")]
  pub(crate) id: String,

  #[serde(rename = "organizationId")]
  pub(crate) organization_id: String,

  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "expiresAt")]
  pub(crate) expires_at: i64,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub(crate) struct PlainApiKey {
  #[serde(rename = "id")]
  pub(crate) id: String,

  #[serde(rename = "organizationId")]
  pub(crate) organization_id: String,

  #[serde(rename = "key")]
  pub(crate) key: String,

  #[serde(rename = "description")]
  pub(crate) description: String,

  #[serde(rename = "expiresAt")]
  pub(crate) expires_at: i64,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub(crate) struct DeleteApiKeyBody {
  #[serde(rename = "id")]
  pub(crate) id: String,

  #[serde(rename = "password")]
  pub(crate) password: String,
}
