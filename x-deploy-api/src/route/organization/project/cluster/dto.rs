use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use validator::Validate;
use x_deploy_common::db::organization_project_cluster::OrganizationProjectCluster;

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
    "status": "CREATING",
    "createdAt": "2020-10-29T15:00:00Z",
}))]
pub struct ClusterInfoResponse {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub cloud_provider: String,
  pub status: String,
  pub created_at: String,
}

impl Into<ClusterInfoResponse> for OrganizationProjectCluster {
  fn into(self) -> ClusterInfoResponse {
    let created_at_str = self
      .id
      .timestamp()
      .to_chrono()
      .to_rfc3339_opts(SecondsFormat::Secs, true);
    ClusterInfoResponse {
      id: self.id.to_string(),
      name: self.name,
      description: self.description,
      cloud_provider: self.cloud_provider,
      status: self.status.to_string(),
      created_at: created_at_str,
    }
  }
}
