use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use x_deploy_common::db::query::organization_member::OrganizationMemberQuery;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(example = json!({
  "id": "5f9f9a9b9f6b9b0001b8e9a0",
  "firstname": "John",
  "lastname": "Doe",
  "email": "john@doe.net",
  "roleId": "5f9f9a9b9f6b9b0001b8e9a0",
  "roleName": "My Custom Role",
  "isOwner": false,
  "since": "2020-11-02T12:00:00Z"
}))]
pub struct MemberInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "firstname")]
  pub firstname: String,

  #[serde(rename = "lastname")]
  pub lastname: String,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "roleId")]
  pub role_id: Option<String>,
  
  #[serde(rename = "roleName")]
  pub role_name: Option<String>,

  #[serde(rename = "isOwner")]
  pub is_owner: bool,

  #[serde(rename = "since")]
  pub since: String,
}

impl From<OrganizationMemberQuery> for MemberInfoResponse {
  fn from(member: OrganizationMemberQuery) -> Self {
    let timestamp: DateTime<Utc> = member.id.timestamp().into();
    let since: String = timestamp.to_string();
    let role_name: Option<String> = match member.role.clone() {
      Some(role) => Some(role.name),
      None => None,
    };
    let role_id: Option<String> = match member.role {
      Some(role) => Some(role.id.to_string()),
      None => None,
    };
    MemberInfoResponse {
      id: member.id.to_string(),
      firstname: member.user.firstname,
      lastname: member.user.lastname,
      is_owner: member.is_owner,
      email: member.user.email.email,
      role_id,
      role_name,
      since,
    }
  }
}
