use crate::error::ApiError;
use crate::guard::auth::Auth;
use crate::permission::get_level;
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use x_deploy_common::db::organization_apikey::OrganizationApiKey;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::{
  OrganizationRole, StandardPermission,
};
use x_deploy_common::db::query::organization_api_key::OrganizationApiKeyQuery;
use x_deploy_common::db::query::organization_member::OrganizationMemberQuery;
use x_deploy_common::db::CommonCollection;

pub enum GeneralPermission {
  Organization,
  Billing,
  Members,
  Project,
  ApiKeys,
  Credentials,
}

impl GeneralPermission {
  pub fn has_general_permission(
    &self,
    role: &Option<OrganizationRole>,
    ask: &StandardPermission,
  ) -> bool {
    return match role {
      None => true,
      Some(role) => {
        let permission = self.get_organization_role(role);
        let level = get_level(&permission);
        let ask_level = get_level(ask);
        return level >= ask_level;
      }
    };
  }

  pub fn verify(
    &self,
    role: &Option<OrganizationRole>,
    ask: &StandardPermission,
  ) -> Result<(), ApiError> {
    let result = self.has_general_permission(role, ask);
    if result {
      return Ok(());
    }
    Err(ApiError::new(
      Status::Forbidden,
      "You don't have the permission to do this".to_string(),
    ))
  }

  /// The method verifies if the user has the permission to
  /// do the action and was a member of the organization.
  /// # Arguments
  ///
  /// * `db`: The database connection
  /// * `auth`: The authentication method
  /// * `org_id`: The id of the organization
  /// * `ask`: The permission to ask for
  ///
  /// returns: Result<Option<ObjectId>, ApiError>
  /// The role id if the user has a role or None if the user is the owner of the organization
  pub async fn verify_auth(
    &self,
    db: &Database,
    auth: Auth,
    org_id: &ObjectId,
    ask: StandardPermission,
  ) -> Result<Option<OrganizationRole>, ApiError> {
    return match auth {
      Auth::ApiKey(api_key) => {
        let result = self
          .verify_key_and_get(db, &api_key.id, &org_id, &ask)
          .await?;
        Ok(result.role)
      }
      Auth::Bearer(token) => {
        let user_id = token.parse_id()?;
        let result = self.verify_and_get(db, &user_id, &org_id, &ask).await?;
        Ok(result.role)
      }
    };
  }

  pub async fn verify_and_get(
    &self,
    db: &Database,
    user_id: &ObjectId,
    org_id: &ObjectId,
    ask: &StandardPermission,
  ) -> Result<OrganizationMemberQuery, ApiError> {
    let omc = CommonCollection::<OrganizationMember>::new(db);
    let org_user = omc.get_user_in_org(org_id, user_id).await?;
    return match org_user {
      Some(org_user) => {
        self.verify(&org_user.role, ask)?;
        Ok(org_user)
      }
      None => Err(ApiError::new(
        Status::NotFound,
        "You are not a member of this organization".to_string(),
      )),
    };
  }

  pub async fn verify_key_and_get(
    &self,
    db: &Database,
    api_key_id: &ObjectId,
    org_id: &ObjectId,
    ask: &StandardPermission,
  ) -> Result<OrganizationApiKeyQuery, ApiError> {
    let oakc = CommonCollection::<OrganizationApiKey>::new(db);
    let api_key = oakc.get_by_id_of_org(org_id, api_key_id).await?;
    return match api_key {
      Some(api_key) => {
        self.verify(&api_key.role, ask)?;
        Ok(api_key)
      }
      None => Err(ApiError::new(
        Status::NotFound,
        "You are not a member of this organization".to_string(),
      )),
    };
  }

  fn get_organization_role(
    &self,
    role: &OrganizationRole,
  ) -> StandardPermission {
    return match self {
      GeneralPermission::ApiKeys => role.general_permission.api_keys.clone(),
      GeneralPermission::Billing => role.general_permission.billing.clone(),
      GeneralPermission::Credentials => {
        role.general_permission.credentials.clone()
      }
      GeneralPermission::Members => role.general_permission.members.clone(),
      GeneralPermission::Organization => {
        role.general_permission.organization.clone()
      }
      GeneralPermission::Project => role.general_permission.project.clone(),
    };
  }
}

#[deprecated]
pub fn has_general_permission(
  role: &OrganizationRole,
  permission: &GeneralPermission,
  ask: &StandardPermission,
) -> bool {
  let permission = GeneralPermission::get_organization_role(permission, role);
  let level = get_level(&permission);
  let ask_level = get_level(ask);
  return level >= ask_level;
}

#[deprecated]
pub fn verify_general_permission(
  role: Option<OrganizationRole>,
  permission: &GeneralPermission,
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
