use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::member::dto::MemberInfoResponse;
use crate::route::{custom_error, custom_response, ApiResult, SuccessMessage};
use crate::utils::object_id::ToObjectId;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::Database;
use rocket::http::Status;
use rocket::State;
use std::str::FromStr;
use std::time::SystemTime;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::StandardPermission;
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
  let members = org_member_coll.get_all_user_in_org(&org_id).await?;
  let mut result: Vec<MemberInfoResponse> = Vec::new();
  for member in members {
    let timestamp: SystemTime = member.id.timestamp().to_system_time();
    let since: DateTime<Utc> = DateTime::from(timestamp);
    let role: Option<String> = match member.role {
      Some(role) => Some(role.name),
      None => None,
    };
    let member_info = MemberInfoResponse {
      id: member.id.to_string(),
      firstname: member.user.firstname,
      lastname: member.user.lastname,
      is_owner: member.is_owner,
      email: member.user.email.email,
      role,
      since: since.to_string(),
    };
    result.push(member_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
  member_id: String,
) -> ApiResult<SuccessMessage> {
  return custom_error(Status::NotImplemented, "Not implemented");
}
