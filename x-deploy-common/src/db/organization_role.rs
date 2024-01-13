use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

const ORGANIZATION_ROLE_COLLECTION_NAME: &str = "organizationRoles";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationRole {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "clusterPermission")]
  pub cluster_permission: ClusterPermission,

  #[serde(rename = "generalPermission")]
  pub general_permission: GeneralPermission,
  // #[serde(rename = "environmentPermissions")]
  // pub environment_permissions: Vec<OrganizationRoleEnvironmentPermission>,
}

// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct OrganizationRoleEnvironmentPermission {
//   #[serde(rename = "environmentId")]
//   pub environment_id: ObjectId,
//
//   #[serde(rename = "permission")]
//   pub permission: EnvironmentPermission,
// }

impl OrganizationRole {
  pub fn new(
    organization_id: ObjectId,
    name: String,
    description: Option<String>,
  ) -> Self {
    let cluster_permission = Default::default();
    let general_permission = Default::default();
    OrganizationRole {
      id: ObjectId::new(),
      organization_id,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum EnvironmentPermission {
  #[serde(rename = "NO_ACCESS")]
  NoAccess,

  #[serde(rename = "READ")]
  Read,

  #[serde(rename = "DEPLOY")]
  Deploy,

  #[serde(rename = "Manage")]
  Manage,

  #[serde(rename = "FULL_ACCESS")]
  FullAccess,
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

  #[serde(rename = "project")]
  pub project: StandardPermission,

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
      project: Default::default(),
      api_keys: Default::default(),
      credentials: Default::default(),
    }
  }
}

impl ToCollectionName for OrganizationRole {
  fn collection_name() -> String {
    String::from(ORGANIZATION_ROLE_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationRole> {
  pub async fn get_of_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationRole>> {
    let filter = doc! {
      "organizationId": org_id,
    };
    let cursor = self.collection.find(filter, None).await?;
    let roles = cursor_to_vec(cursor).await?;
    Ok(roles)
  }

  pub async fn get_with_id_of_org(
    &self,
    org_id: &ObjectId,
    role_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationRole>> {
    let filter = doc! {
      "_id": role_id,
      "organizationId": org_id,
    };
    let result = self.collection.find_one(filter, None).await?;
    Ok(result)
  }
}
