use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::role::dto::{
  CreateCustomRoleRequest, CustomRoleInfoResponse, UpdateCustomRoleRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use bson::oid::ObjectId;
use mongodb::results::DeleteResult;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_invitation::OrganizationInvitation;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::GeneralPermission as CommonGeneralPermission;
use x_deploy_common::db::organization_role::{
  ClusterPermission, OrganizationRole, StandardPermission,
};
use x_deploy_common::db::CommonCollection;

pub(crate) async fn new(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
  body: Json<CreateCustomRoleRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Roles
    .verify_and_get(database, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  let collection = CommonCollection::<OrganizationRole>::new(database);
  let to_insert: OrganizationRole = body.into_inner().into();
  collection.insert_one(&to_insert).await?;

  custom_message(Status::Ok, "Role created successfully for the organization")
}

pub(crate) async fn all(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<CustomRoleInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Roles
    .verify_and_get(database, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  let collection = CommonCollection::<OrganizationRole>::new(database);
  let db_result: Vec<OrganizationRole> = collection.get_of_org(&org_id).await?;
  let mut response = Vec::<CustomRoleInfoResponse>::new();
  for role in db_result {
    let role: CustomRoleInfoResponse = role.into();
    response.push(role);
  }
  custom_response(Status::Ok, response)
}

pub(crate) async fn get_by_id(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
  role_id: &str,
) -> ApiResult<CustomRoleInfoResponse> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let role_id = ObjectId::from_str(role_id)?;

  GeneralPermission::Roles
    .verify_and_get(database, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  let collection: CommonCollection<OrganizationRole> =
    CommonCollection::new(database);
  let result: Option<OrganizationRole> =
    collection.get_with_id_and_org(&org_id, &role_id).await?;
  return match result {
    None => custom_error(Status::NotFound, "Role not found"),
    Some(role) => {
      let response: CustomRoleInfoResponse = role.into();
      custom_response(Status::Ok, response)
    }
  };
}

pub(crate) async fn delete(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
  role_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let role_id = ObjectId::from_str(role_id)?;

  GeneralPermission::Roles
    .verify_and_get(database, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  let role_collection = CommonCollection::<OrganizationRole>::new(database);
  let role: Option<OrganizationRole> = role_collection
    .get_with_id_and_org(&org_id, &role_id)
    .await?;
  if let None = role {
    return custom_error(
      Status::BadRequest,
      "Role not found in the organization",
    );
  }

  // Check if any member has this role
  let member_collection = CommonCollection::<OrganizationMember>::new(database);
  let members = member_collection.get_with_role(&org_id, &role_id).await?;
  if members.len() > 0 {
    let error_message = format!(
      "Cannot delete role because it is still in use by {} members",
      members.len()
    );
    return custom_error(Status::BadRequest, error_message.as_str());
  }
  // Check if any invitation has this role
  let invitation_collection =
    CommonCollection::<OrganizationInvitation>::new(database);
  let invitations = invitation_collection
    .get_with_role(&org_id, &role_id)
    .await?;
  if invitations.len() > 0 {
    let error_message = format!(
      "Cannot delete role because it is still in use by {} invitations",
      members.len()
    );
    return custom_error(Status::BadRequest, error_message.as_str());
  }
  let deleted: DeleteResult = role_collection.delete_by_id(&role_id).await?;
  if deleted.deleted_count == 0 {
    return custom_error(
      Status::NotFound,
      "Fail to delete role du to database error",
    );
  }
  custom_message(Status::Ok, "Role deleted successfully")
}

pub(crate) async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  role_id: &str,
  body: Json<UpdateCustomRoleRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let role_id = ObjectId::from_str(role_id)?;

  let org_user = GeneralPermission::Roles
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;
  if let Some(user_role) = org_user.role {
    if user_role.id == role_id {
      return custom_error(
        Status::BadRequest,
        "Cannot update your own role. Please ask another admin to do it",
      );
    }
  }

  let role_collection = CommonCollection::<OrganizationRole>::new(db);
  let role: Option<OrganizationRole> = role_collection
    .get_with_id_and_org(&org_id, &role_id)
    .await?;
  if let None = role {
    return custom_error(
      Status::BadRequest,
      "Role not found in the organization",
    );
  }
  // Update Info
  role_collection
    .update_info(&org_id, &role_id, &body.name, body.description.clone())
    .await?;
  // Update cluster permission
  let cluster_permission: ClusterPermission =
    body.cluster_permission.clone().into();
  role_collection
    .update_cluster_permission(&org_id, &role_id, &cluster_permission)
    .await?;
  // Update general permission
  let general_permission: CommonGeneralPermission = body.into_inner().into();
  role_collection
    .update_general_permission(&org_id, &role_id, &general_permission)
    .await?;
  // Send response
  custom_message(Status::Ok, "Role updated successfully")
}
