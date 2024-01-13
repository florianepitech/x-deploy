use rocket::serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "My Ovh Credentials",
  "description": "My Ovh Credentials description",
  "applicationKey": "7kbG7Bk7S9Nt7ZSV",
  "applicationSecret": "EXEgWIz07P0HYwtQDs7cNIqCiQaWSuHF",
  "consumerKey": "MtSwSrPpNjqfVSmJhLbPyr2i45lSwPU1",
}))]
pub struct NewOvhCredentialsRequest {
  pub name: String,
  pub description: Option<String>,
  pub application_key: String,
  pub application_secret: String,
  pub consumer_key: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "id": "5f9b3b2b9d3f6c0007f7e7b1",
  "name": "My Ovh Credentials",
  "description": "My Ovh Credentials description",
  "created_at": "2030-10-30T14:00:00.000Z",
}))]
pub struct OvhCredentialsInfoResponse {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "My Ovh Credentials",
  "description": "My Ovh Credentials description",
}))]
pub struct UpdateOvhCredentialsRequest {
  pub name: String,
  pub description: Option<String>,
}
