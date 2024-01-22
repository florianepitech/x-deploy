use crate::db::organization_member::OrganizationMember;
use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::oid::ObjectId;
use bson::{doc, Bson};
use log::info;
use mongodb::results::{DeleteResult, UpdateResult};
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
}

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

impl Default for ClusterPermission {
  fn default() -> Self {
    ClusterPermission::ReadEnvironment
  }
}

impl From<ClusterPermission> for Bson {
  fn from(permission: ClusterPermission) -> Self {
    match permission {
      ClusterPermission::FullAccess => Bson::String("FULL_ACCESS".to_string()),
      ClusterPermission::CreateEnvironment => {
        Bson::String("CREATE_ENVIRONMENT".to_string())
      }
      ClusterPermission::ReadEnvironment => {
        Bson::String("READ_ENVIRONMENT".to_string())
      }
    }
  }
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

impl From<StandardPermission> for Bson {
  fn from(permission: StandardPermission) -> Self {
    match permission {
      StandardPermission::None => Bson::String("NONE".to_string()),
      StandardPermission::Read => Bson::String("READ".to_string()),
      StandardPermission::ReadWrite => Bson::String("READ_WRITE".to_string()),
    }
  }
}

impl Default for StandardPermission {
  fn default() -> Self {
    StandardPermission::None
  }
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

impl Default for EnvironmentPermission {
  fn default() -> Self {
    EnvironmentPermission::NoAccess
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

  #[serde(rename = "role")]
  pub role: StandardPermission,
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
      role: Default::default(),
    }
  }
}

impl From<GeneralPermission> for Bson {
  fn from(value: GeneralPermission) -> Self {
    return Bson::from(doc! {
      "organization": value.organization,
      "billing": value.billing,
      "members": value.members,
      "project": value.project,
      "apiKeys": value.api_keys,
      "credentials": value.credentials,
      "role": value.role,
    });
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

  pub async fn delete_of_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<DeleteResult> {
    let filter = doc! {
      "organizationId": org_id,
    };
    let result = self.collection.delete_many(filter, None).await?;
    Ok(result)
  }
  
  pub async fn get_with_id_and_org(
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

  pub async fn update_general_permission(
    &self,
    org_id: &ObjectId,
    role_id: &ObjectId,
    permission: &GeneralPermission,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": role_id,
      "organizationId": org_id,
    };
    let update = doc! {
      "$set": {
        "generalPermission": permission,
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }

  pub async fn update_cluster_permission(
    &self,
    org_id: &ObjectId,
    role_id: &ObjectId,
    permission: &ClusterPermission,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": role_id,
      "organizationId": org_id,
    };
    let update = doc! {
      "$set": {
        "clusterPermission": permission,
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }

  pub async fn update_info(
    &self,
    org_id: &ObjectId,
    role_id: &ObjectId,
    name: &str,
    description: Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": role_id,
      "organizationId": org_id,
    };
    let bson_description = match description {
      Some(description) => bson::Bson::String(description),
      None => bson::Bson::Null,
    };
    let update = doc! {
      "$set": {
        "name": name,
        "description": bson_description,
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }
}
