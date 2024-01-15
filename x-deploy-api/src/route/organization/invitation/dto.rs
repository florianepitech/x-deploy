use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use x_deploy_common::db::organization_invitation::OrganizationInvitation;
use x_deploy_common::db::query::organization_invitation::OrganizationInvitationQuery;

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

impl From<OrganizationInvitationQuery> for OrganizationInvitationInfoResponse {
  fn from(value: OrganizationInvitationQuery) -> Self {
    let send_at = value.id.timestamp().to_chrono().to_string();
    let response_at = match value.response_at {
      Some(response_at) => Some(response_at.to_string()),
      None => None,
    };
    OrganizationInvitationInfoResponse {
      sender: OrganizationInvitationInfoUser {
        id: value.sender.id.to_string(),
        firstname: value.sender.firstname,
        lastname: value.sender.lastname,
        email: value.sender.email.email,
      },
      receiver: OrganizationInvitationInfoUser {
        id: value.receiver.id.to_string(),
        firstname: value.receiver.firstname,
        lastname: value.receiver.lastname,
        email: value.receiver.email.email,
      },
      status: value.status.to_string(),
      sent_at: send_at,
      response_at,
    }
  }
}
