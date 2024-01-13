use crate::guard::bearer_token::BearerToken;
use crate::route::cloud_provider::aws::dto::{
  CloudProviderAwsInstance, CloudProviderAwsRegion,
};
use crate::route::ApiResult;

mod controller;
pub mod dto;

#[utoipa::path(
    get,
    operation_id = "Get All Region",
    path = "/cloud-provider/aws/region",
    tag = "Cloud Provider AWS",
    responses(
        (status = 200, description = "Get all availble region", body = Vec<CloudProviderAwsRegion>),
    ),
)]
#[get("/cloud-provider/aws/region", format = "application/json")]
pub async fn all_region(
  token: BearerToken
) -> ApiResult<Vec<CloudProviderAwsRegion>> {
  controller::all_region().await
}

#[deprecated]
#[utoipa::path(
    get,
    operation_id = "Get All Instance",
    path = "/cloud-provider/aws/instance",
    tag = "Cloud Provider AWS",
    responses(
        (status = 200, description = "Get all available instance", body = Vec<CloudProviderAwsInstance>),
    ),
)]
#[get("/cloud-provider/aws/instance", format = "application/json")]
pub async fn instance_types(
  token: BearerToken
) -> ApiResult<Vec<CloudProviderAwsInstance>> {
  controller::instance_types().await
}
