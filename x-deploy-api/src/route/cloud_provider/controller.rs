use crate::guard::bearer_token::BearerToken;
use crate::route::cloud_provider::controller;
use crate::route::cloud_provider::dto::{
  CloudProviderResponse, CloudProviderType,
};
use crate::route::{custom_response, ApiResult};
use mongodb::Database;
use rocket::http::Status;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

pub(crate) async fn all(
  token: BearerToken
) -> ApiResult<Vec<CloudProviderResponse>> {
  let vec = vec![
    CloudProviderType::Aws,
    CloudProviderType::GoogleCloud,
    CloudProviderType::Azure,
    CloudProviderType::Ovh,
  ];
  let mut result: Vec<CloudProviderResponse> = Vec::new();
  for item in vec {
    let cloud_provider = CloudProviderResponse {
      name: item.to_string(),
    };
    result.push(cloud_provider);
  }
  return custom_response(Status::Ok, result);
}
