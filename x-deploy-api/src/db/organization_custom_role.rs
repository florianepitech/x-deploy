use bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

pub(crate) const ORGANIZATION_ROLE_COLLECTION_NAME: &str = "organizationRoles";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationRole {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "clusterPermission")]
  pub cluster_permission: ClusterPermission,

  #[serde(rename = "generalPermission")]
  pub general_permission: GeneralPermission,
}

impl OrganizationRole {
  pub fn new(
    id: ObjectId,
    name: String,
    description: String,
  ) -> Self {
    let cluster_permission = Default::default();
    let general_permission = Default::default();
    OrganizationRole {
      id,
      name,
      description,
      cluster_permission,
      general_permission,
    }
  }
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum StandardPermission {
  #[serde(rename = "NONE")]
  None,

  #[serde(rename = "READ")]
  Read,

  #[serde(rename = "READ_WRITE")]
  ReadWrite,
}

impl Default for ClusterPermission {
  fn default() -> Self {
    ClusterPermission::ReadEnvironment
  }
}

impl Default for StandardPermission {
  fn default() -> Self {
    StandardPermission::None
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GeneralPermission {
  #[serde(rename = "organization")]
  pub organization: StandardPermission,

  #[serde(rename = "billing")]
  pub billing: StandardPermission,

  #[serde(rename = "members")]
  pub members: StandardPermission,

  #[serde(rename = "apiKeys")]
  pub api_keys: StandardPermission,

  #[serde(rename = "credentials")]
  pub credentials: StandardPermission,
}

impl Default for GeneralPermission {
  fn default() -> Self {
    GeneralPermission {
      organization: Default::default(),
      billing: Default::default(),
      members: Default::default(),
      api_keys: Default::default(),
      credentials: Default::default(),
    }
  }
}
