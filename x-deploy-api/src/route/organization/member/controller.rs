use crate::db::query::organization_member::query_organization_member_get_all_in_org;
use crate::guard::token::Token;
use crate::route::organization::member::dto::MemberInfoResponse;
use crate::route::{
  custom_error, custom_response, ApiResponse, SuccessMessage,
};
use bson::oid::ObjectId;
use chrono::DateTime;
use mongodb::Database;
use rocket::http::Status;
use rocket::State;
use std::str::FromStr;
use std::time::SystemTime;

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResponse<Vec<MemberInfoResponse>> {
  let org_id = match ObjectId::from_str(&org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Organization id is not a valid id",
      )
    }
  };
  let members = query_organization_member_get_all_in_org(db, &org_id).await?;
  let mut result: Vec<MemberInfoResponse> = Vec::new();
  for member in members {
    let timestamp: SystemTime = member.id.timestamp().to_system_time();
    let since = DateTime::from(timestamp);
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
      since,
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
) -> ApiResponse<SuccessMessage> {
  return custom_error(Status::NotImplemented, "Not implemented");
}
