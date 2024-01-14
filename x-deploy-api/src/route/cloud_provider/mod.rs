use crate::guard::bearer_token::BearerToken;
use crate::route::cloud_provider::dto::CloudProviderResponse;
use crate::route::ApiResult;
use mongodb::Database;
use rocket::State;

pub mod aws;
mod controller;
pub mod dto;
mod ovh;

#[utoipa::path(
    get,
    operation_id = "Get all cloud provider available",
    path = "/cloud-provider",
    tag = "Cloud Provider",
    security(
        ("bearer" = []),
    ),
    responses(
        (status = 200, description = "The list of cloud provider available", body = Vec<CloudProviderResponse>),
    ),
)]
#[get("/cloud-provider", format = "application/json")]
pub async fn all(token: BearerToken) -> ApiResult<Vec<CloudProviderResponse>> {
  controller::all(token).await
}
