use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct CreateApiKeyRequest {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  /// If null, key has all permissions
  #[serde(rename = "roleId")]
  pub role_id: Option<String>,

  /// If null, key will never expire
  #[serde(rename = "expiresAt")]
  pub expires_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct CreateApiKeyResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "key")]
  pub key: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct ApiKeyInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "organizationId")]
  pub organization_id: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "role")]
  pub role: Option<ApiKeyRoleInfoResponse>,

  #[serde(rename = "createdAt")]
  pub created_at: String,

  #[serde(rename = "expiresAt")]
  pub expires_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct ApiKeyRoleInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct DeleteApiKeyRequest {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "password")]
  pub password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct UpdateApiKeyRequest {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,
}
