use crate::error::ApiError;
use crate::permission::get_level;
use rocket::http::Status;
use x_deploy_common::db::organization_role::{
  OrganizationRole, StandardPermission,
};

pub enum GeneralPermissionType {
  Organization,
  Billing,
  Members,
  ApiKeys,
  Credentials,
}

pub fn has_general_permission(
  role: &OrganizationRole,
  permission: &GeneralPermissionType,
  ask: &StandardPermission,
) -> bool {
  let permission =
    GeneralPermissionType::from_organization_role(permission, role);
  let level = get_level(&permission);
  let ask_level = get_level(ask);
  return level >= ask_level;
}

pub fn verify_general_permission(
  role: Option<OrganizationRole>,
  permission: &GeneralPermissionType,
  ask: &StandardPermission,
) -> Result<(), ApiError> {
  return match role {
    None => Ok(()),
    Some(role) => {
      let result = has_general_permission(&role, permission, ask);
      if result {
        return Ok(());
      }
      Err(ApiError::new(
        Status::Forbidden,
        "You don't have the permission to do this".to_string(),
      ))
    }
  };
}

impl GeneralPermissionType {
  fn from_organization_role(
    permission_type: &GeneralPermissionType,
    role: &OrganizationRole,
  ) -> StandardPermission {
    return match permission_type {
      GeneralPermissionType::ApiKeys => {
        role.general_permission.api_keys.clone()
      }
      GeneralPermissionType::Billing => role.general_permission.billing.clone(),
      GeneralPermissionType::Credentials => {
        role.general_permission.credentials.clone()
      }
      GeneralPermissionType::Members => role.general_permission.members.clone(),
      GeneralPermissionType::Organization => {
        role.general_permission.organization.clone()
      }
    };
  }
}
