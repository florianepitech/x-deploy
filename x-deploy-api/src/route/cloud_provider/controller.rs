use crate::guard::bearer_token::BearerToken;
use crate::route::cloud_provider::dto::CloudProviderResponse;
use crate::route::{custom_response, ApiResult};
use rocket::http::Status;
use x_deploy_common::data::cloud_provider::CloudProviderType;

pub(crate) async fn all(
  token: BearerToken
) -> ApiResult<Vec<CloudProviderResponse>> {
  let vec = CloudProviderType::get_all();
  let mut result: Vec<CloudProviderResponse> = Vec::new();
  for item in vec {
    let cloud_provider = CloudProviderResponse {
      name: item.to_string(),
    };
    result.push(cloud_provider);
  }
  return custom_response(Status::Ok, result);
}
