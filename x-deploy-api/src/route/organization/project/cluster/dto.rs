use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "name": "My Cluster",
    "description": "My Cluster Description",
    "cloudProvider": "AWS",
    "credentialId": "5f9b3b3b9c6d2b0007f1b3b3"
}))]
pub struct CreateClusterRequest {
  pub name: String,
  pub description: Option<String>,
  pub cloud_provider: String,
  pub credential_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
    "id": "5f9b3b3b9c6d2b0007f1b3b3",
    "name": "My Cluster",
    "description": "My Cluster Description",
    "cloudProvider": "AWS",
}))]
pub struct ClusterInfoResponse {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub cloud_provider: String,
  pub status: String,
}
