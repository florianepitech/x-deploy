use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use x_deploy_common::db::organization_role::{
  ClusterPermission, GeneralPermission, OrganizationRole, StandardPermission,
};

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[schema(example = json!({
    "name": "My new role name",
    "description": "My new role description"
}))]
pub struct CreateCustomRoleRequest {
  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[schema(example = json!({
    "id": "5f9b3b7b9c6b2b0007f1e7b2",
    "name": "My new role name",
    "description": "My new role description",
    "clusterPermission": "FULL_ACCESS"
}))]
pub struct CustomRoleInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "clusterPermission")]
  pub cluster_permission: CustomRoleClusterPermissionInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[schema(example = json!({
    "name": "My new role name",
    "description": "My new role description",
    "clusterPermission": "FULL_ACCESS",
    "generalOrganization": "READ_WRITE",
    "generalBilling": "READ_WRITE",
    "generalMembers": "READ_WRITE",
    "generalMembers": "READ_WRITE",
    "generalProject": "READ_WRITE",
    "generalApiKeys": "READ_WRITE",
    "generalCredentials": "READ_WRITE",
    "generalRole": "READ_WRITE"
}))]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomRoleRequest {
  // Info
  pub name: String,
  pub description: Option<String>,
  // Cluster permission
  pub cluster_permission: CustomRoleClusterPermissionInfo,
  // General permission
  pub general_organization: CustomRoleGeneralPermissionInfo,
  pub general_billing: CustomRoleGeneralPermissionInfo,
  pub general_members: CustomRoleGeneralPermissionInfo,
  pub general_project: CustomRoleGeneralPermissionInfo,
  pub general_api_keys: CustomRoleGeneralPermissionInfo,
  pub general_credentials: CustomRoleGeneralPermissionInfo,
  pub general_role: CustomRoleGeneralPermissionInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[schema(example = json!("FULL_ACCESS"))]
pub enum CustomRoleClusterPermissionInfo {
  FullAccess,

  CreateEnvironment,

  ReadEnvironment,
}

impl Into<ClusterPermission> for CustomRoleClusterPermissionInfo {
  fn into(self) -> ClusterPermission {
    match self {
      CustomRoleClusterPermissionInfo::FullAccess => {
        ClusterPermission::FullAccess
      }
      CustomRoleClusterPermissionInfo::CreateEnvironment => {
        ClusterPermission::CreateEnvironment
      }
      CustomRoleClusterPermissionInfo::ReadEnvironment => {
        ClusterPermission::ReadEnvironment
      }
    }
  }
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[schema(example = json!("READ_WRITE"))]
pub enum CustomRoleGeneralPermissionInfo {
  None,
  Read,
  ReadWrite,
}

impl Into<OrganizationRole> for CreateCustomRoleRequest {
  fn into(self) -> OrganizationRole {
    OrganizationRole {
      id: Default::default(),
      name: self.name,
      description: self.description,
      organization_id: Default::default(),
      cluster_permission: Default::default(),
      general_permission: Default::default(),
    }
  }
}

impl Into<GeneralPermission> for UpdateCustomRoleRequest {
  fn into(self) -> GeneralPermission {
    GeneralPermission {
      organization: self.general_organization.into(),
      billing: self.general_billing.into(),
      members: self.general_members.into(),
      project: self.general_project.into(),
      api_keys: self.general_api_keys.into(),
      credentials: self.general_credentials.into(),
      role: self.general_role.into(),
    }
  }
}

impl Into<CustomRoleInfoResponse> for OrganizationRole {
  fn into(self) -> CustomRoleInfoResponse {
    CustomRoleInfoResponse {
      id: self.id.to_string(),
      name: self.name,
      description: self.description,
      cluster_permission: CustomRoleClusterPermissionInfo::FullAccess,
    }
  }
}

impl Into<StandardPermission> for CustomRoleGeneralPermissionInfo {
  fn into(self) -> StandardPermission {
    match self {
      CustomRoleGeneralPermissionInfo::None => StandardPermission::None,
      CustomRoleGeneralPermissionInfo::Read => StandardPermission::Read,
      CustomRoleGeneralPermissionInfo::ReadWrite => {
        StandardPermission::ReadWrite
      }
    }
  }
}
