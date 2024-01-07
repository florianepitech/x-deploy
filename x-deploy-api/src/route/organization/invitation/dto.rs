use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(example = json!({
  "email": "john@doe.net",
  "roleId": "5f9b3b3b9b7e4b0001f1e3b0",
}))]
#[serde(rename_all = "camelCase")]
pub struct NewOrganizationInvitationRequest {
  pub email: String,
  pub role_id: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationInvitationInfoResponse {
  pub sender: OrganizationInvitationInfoUser,
  pub receiver: OrganizationInvitationInfoUser,
  pub status: String,
  pub sent_at: String,
  pub response_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct OrganizationInvitationInfoUser {
  pub id: String,
  pub firstname: String,
  pub lastname: String,
  pub email: String,
}
