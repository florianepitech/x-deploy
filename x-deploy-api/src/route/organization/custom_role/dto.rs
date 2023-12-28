use bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct CreateCustomRoleBody {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct CustomRoleInfoResponse {
  #[serde(rename = "id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "clusterPermission")]
  pub cluster_permission: CustomRoleClusterPermissionInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub enum CustomRoleClusterPermissionInfo {
  #[serde(rename = "fullAccess")]
  FullAccess,

  #[serde(rename = "createEnvironment")]
  CreateEnvironment,

  #[serde(rename = "readEnvironment")]
  ReadEnvironment,
}
