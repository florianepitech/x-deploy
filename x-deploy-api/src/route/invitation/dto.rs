use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "5f9b2b3c4d5e6f7a8b9c0d1e",
    "sender": {
        "id": "5f9b2b3c4d5e6f7a8b9c0d1e",
        "firstname": "John",
        "lastname": "Doe",
        "email": "john@doe.net",
    },
    "organization": {
        "id": "5f9b2b3c4d5e6f7a8b9c0d1e",
        "name": "My organization",
        "description": "My organization description",
        "website": "https://my-organization.com"
    }
}))]
pub struct InvitationInfoResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "sender")]
  pub sender: InvitationInfoUser,

  #[serde(rename = "organization")]
  pub organization: InvitationInfoOrganization,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "response": true
}))]
pub struct InvitationResponseRequest {
  #[serde(rename = "response")]
  pub response: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "5f9b2b3c4d5e6f7a8b9c0d1e",
    "firstname": "John",
    "lastname": "Doe",
    "email": "john@doe.net",
}))]
pub struct InvitationInfoUser {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "firstname")]
  pub firstname: String,

  #[serde(rename = "lastname")]
  pub lastname: String,

  #[serde(rename = "email")]
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "5f9b2b3c4d5e6f7a8b9c0d1e",
    "name": "My organization",
    "description": "My organization description",
    "website": "https://my-organization.com",
}))]
pub struct InvitationInfoOrganization {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "website")]
  pub website: String,
}
