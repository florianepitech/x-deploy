use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "name": "My Stunning Organization",
    "description": "A new amazing organization !",
    "role_id": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "expires_at": "2021-01-01T00:00:00Z",
}))]
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
#[schema(example = json!({
    "id": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "key": "hhqtOgA7boOx9hK...",
}))]
pub struct CreateApiKeyResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "key")]
  pub key: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "id": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "organizationId": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "name": "My Stunning API Key",
    "description": "A new amazing API Key !",
    "role": {
        "id": "5f7b1b1b1b1b1b1b1b1b1b1b",
        "name": "Admin",
        "description": "Admin role"
    },
    "createdAt": "2020-10-05T00:00:00Z",
    "expiresAt": "2021-01-01T00:00:00Z",
}))]
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
#[schema(example = json!({
    "id": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "name": "Admin",
    "description": "Admin role",
}))]
pub struct ApiKeyRoleInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "id": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "password": "MyAmazingPassword123!",
}))]
pub struct DeleteApiKeyRequest {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "password")]
  pub password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "id": "5f7b1b1b1b1b1b1b1b1b1b1b",
    "name": "New Role Admin Name",
    "description": "New Role Admin Description",
}))]
pub struct UpdateApiKeyRequest {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,
}
