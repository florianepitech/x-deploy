use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct NewInvitationRequest {
  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "role")]
  pub role: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct InvitationInfoResponse {
  #[serde(rename = "email")]
  pub sender: InvitationInfoUser,

  #[serde(rename = "receiver")]
  pub receiver: InvitationInfoUser,

  #[serde(rename = "status")]
  pub status: String,

  #[serde(rename = "sentAt")]
  pub sent_at: DateTime<Utc>,

  #[serde(rename = "responseAt")]
  pub response_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct InvitationInfoUser {
  #[serde(rename = "email")]
  pub firstname: String,

  #[serde(rename = "lastname")]
  pub lastname: String,
}
