use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(example = json!({
  "id": "5f9f9a9b9f6b9b0001b8e9a0",
  "firstname": "John",
  "lastname": "Doe",
  "email": "john@doe.net",
  "role": "My Custom Role",
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

  #[serde(rename = "role")]
  pub role: Option<String>,

  #[serde(rename = "isOwner")]
  pub is_owner: bool,

  #[serde(rename = "since")]
  pub since: String,
}
