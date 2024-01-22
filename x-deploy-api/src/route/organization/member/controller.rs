use crate::guard::auth::Auth;
use crate::permission::general::GeneralPermission;
use crate::route::organization::member::dto::MemberInfoResponse;
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::query::organization_member::OrganizationMemberQuery;
use x_deploy_common::db::CommonCollection;

pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<MemberInfoResponse>> {
  let org_id = ObjectId::from_str(org_id)?;
  // Verify user is in the organization and have the permission to read
  GeneralPermission::Members
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;
  // Get all members
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let members: Vec<OrganizationMemberQuery> =
    org_member_coll.get_all_user_in_org(&org_id).await?;
  let mut result: Vec<MemberInfoResponse> = Vec::new();
  for member in members {
    let member_info: MemberInfoResponse = member.into();
    result.push(member_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  member_id: &str,
) -> ApiResult<MemberInfoResponse> {
  let org_id = ObjectId::from_str(org_id)?;
  let member_id = ObjectId::from_str(member_id)?;

  // Verify user is in the organization and have the permission to read
  GeneralPermission::Members
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;
  // Get member
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let member: Option<OrganizationMemberQuery> =
    org_member_coll.get_user_in_org(&org_id, &member_id).await?;
  return match member {
    Some(member) => custom_response(Status::Ok, member.into()),
    None => custom_error(Status::NotFound, "Member not found"),
  };
}

pub(crate) async fn delete(
  db: &State<Database>,
  auth: Auth,
  org_id: String,
  member_id: String,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(&org_id)?;
  let member_id = ObjectId::from_str(&member_id)?;
  // Verify user is in the organization and have the permission to read
  GeneralPermission::Members
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;
  let collection = CommonCollection::<OrganizationMember>::new(db);
  let delete_result =
    collection.delete_by_id_and_org(&member_id, &org_id).await?;
  if delete_result.deleted_count == 0 {
    return custom_error(Status::NotFound, "Member not found");
  }
  return custom_message(Status::Ok, "Member was removed from organization");
}
