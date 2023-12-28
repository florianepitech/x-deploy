use bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

pub(crate) const ORGANIZATION_CUSTOM_ROLE_COLLECTION_NAME: &str =
  "organizationCustomRoles";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationCustomRole {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "clusterPermission")]
  pub cluster_permission: ClusterPermission,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ClusterPermission {
  #[serde(rename = "FULL_ACCESS")]
  FullAccess,

  #[serde(rename = "CREATE_ENVIRONMENT")]
  CreateEnvironment,

  #[serde(rename = "READ_ENVIRONMENT")]
  ReadEnvironment,
}
