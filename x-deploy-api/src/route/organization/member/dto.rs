use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct MemberInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "firstname")]
  pub firstname: String,

  #[serde(rename = "lastname")]
  pub lastname: String,

  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "role")]
  pub role: Option<String>,

  #[serde(rename = "isOwner")]
  pub is_owner: bool,

  #[serde(rename = "since")]
  pub since: DateTime<Utc>,
}
