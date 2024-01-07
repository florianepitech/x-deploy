use rocket::serde::json::serde_json::json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "eu-west-1",
  "endpoint": "ec2.eu-west-1.amazonaws.com"
}))]
pub struct CloudProviderAwsRegion {
  pub name: String,
  pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json!({
  "name": "t2.micro"
}))]
pub struct CloudProviderAwsInstance {
  pub name: String,
}
