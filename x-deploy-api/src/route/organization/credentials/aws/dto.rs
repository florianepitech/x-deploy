use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "My Aws Credentials",
  "description": "My Aws Credentials description",
  "accessKey": "AKIAIOSFODNN7EXAMPLE",
  "secretKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
}))]
pub struct NewAwsCredentialsRequest {
  pub name: String,
  pub description: Option<String>,
  pub access_key: String,
  pub secret_key: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "id": "5f9b3b2b9d3f6c0007f7e7b1",
  "name": "My Aws Credentials",
  "description": "My Aws Credentials description",
  "created_at": "2020-10-30T14:30:51.000Z"
}))]
pub struct AwsCredentialsInfoResponse {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "My Aws Credentials",
  "description": "My Aws Credentials description",
}))]
pub struct UpdateAwsCredentialsRequest {
  pub name: String,
  pub description: Option<String>,
}
