mod controller;
pub(crate) mod dto;

use crate::guard::bearer_token::BearerToken;
use crate::route::organization::api_key::dto::{
  ApiKeyInfoResponse, CreateApiKeyRequest, CreateApiKeyResponse,
};
use crate::route::{ApiResult, SuccessMessage};
use bson::doc;
use dto::UpdateApiKeyRequest;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

#[deprecated]
#[utoipa::path(
    post,
    operation_id = "Create ApiKey",
    path = "/organization/{id}/api-key",
    tag = "Organization ApiKey",
    responses(
        (status = 200, description = "Your api key has been created", body = SuccessMessage)
    ),
    request_body = CreateApiKeyRequest,
)]
#[post(
  "/organization/<id>/api-key",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  id: String,
  body: Json<CreateApiKeyRequest>,
) -> ApiResult<CreateApiKeyResponse> {
  controller::new(db, token, id, body).await
}

#[deprecated]
#[utoipa::path(
    get,
    operation_id = "Get ApiKey",
    path = "/organization/<id>/api-key",
    tag = "Organization ApiKey",
    responses(
        (status = 200, description = "Api key retrieved", body = Vec<ApiKeyInfoResponse>),
    )
)]
#[get("/organization/<id>/api-key", format = "application/json")]
pub(crate) async fn get(
  db: &State<Database>,
  token: BearerToken,
  id: String,
) -> ApiResult<Vec<ApiKeyInfoResponse>> {
  controller::get(db, token, id).await
}

#[deprecated]
#[utoipa::path(
    get,
    operation_id = "Get ApiKey by Id",
    path = "/organization/<id>/api-key/<key_id>",
    tag = "Organization ApiKey",
    responses(
        (status = 200, description = "Specific api key retrieved", body = SuccessMessage),
    )
)]
#[get("/organization/<id>/api-key/<key_id>", format = "application/json")]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: BearerToken,
  id: String,
  key_id: String,
) -> ApiResult<SuccessMessage> {
  controller::get_by_id(db, token, id, key_id).await
}

#[deprecated]
#[utoipa::path(
    get,
    operation_id = "Update ApiKey Info",
    path = "/organization/<id>/api-key/<key_id>",
    tag = "Organization ApiKey",
    responses(
        (status = 200, description = "Api key info updated", body = SuccessMessage),
    )
)]
#[patch(
  "/organization/<org_id>/api-key/<key_id>",
  format = "application/json",
  data = "<body>"
)]
pub async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
  key_id: String,
  body: Json<UpdateApiKeyRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, token, org_id, key_id, body).await
}

#[deprecated]
#[utoipa::path(
    delete,
    operation_id = "Delete ApiKey",
    path = "/organization/<id>/api-key/<key_id>",
    tag = "Organization ApiKey",
    responses(
        (status = 200, description = "Api key deleted", body = SuccessMessage),
    )
)]
#[delete("/organization/<id>/api-key/<key_id>", format = "application/json")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  id: String,
  key_id: String,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, token, id, key_id).await
}
