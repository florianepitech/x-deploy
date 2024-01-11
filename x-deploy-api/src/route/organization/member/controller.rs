use crate::guard::token::Token;
use crate::route::organization::member::dto::MemberInfoResponse;
use crate::route::{custom_error, custom_response, ApiResult, SuccessMessage};
use crate::utils::object_id::ToObjectId;
use chrono::{DateTime, Utc};
use mongodb::Database;
use rocket::http::Status;
use rocket::State;
use std::time::SystemTime;
use x_deploy_common::db::organization_member::OrganizationMember;

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResult<Vec<MemberInfoResponse>> {
  let _ = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let members = OrganizationMember::get_all_user_in_org(db, &org_id).await?;
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
  token: Token,
  org_id: String,
  member_id: String,
) -> ApiResult<SuccessMessage> {
  return custom_error(Status::NotImplemented, "Not implemented");
}
