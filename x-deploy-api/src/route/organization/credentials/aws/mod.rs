mod controller;
pub mod dto;

use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::route::{ApiResult, SuccessMessage};
use dto::{
  AwsCredentialsInfoResponse, NewAwsCredentialsRequest,
  UpdateAwsCredentialsRequest,
};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

#[utoipa::path(
  post,
  operation_id = "Create Aws Credential",
  path = "/organization/<org_id>/credentials/aws",
  tag = "Organization Credentials Aws",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 201, description = "Successfully created new Aws credential", body = SuccessMessage),
  ),
  request_body = NewAwsCredentialsRequest,
)]
#[post(
  "/organization/<org_id>/credentials/aws",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  body: Json<NewAwsCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, auth, org_id, body).await
}

#[utoipa::path(
  get,
  operation_id = "Get Aws Credentials",
  path = "/organization/<org_id>/credentials/aws/<cred_id>",
  tag = "Organization Credentials Aws",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Get Aws credential", body = AwsCredentialsInfoResponse),
  ),
)]
#[get(
  "/organization/<org_id>/credentials/aws/<cred_id>",
  format = "application/json"
)]
pub(crate) async fn get(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<AwsCredentialsInfoResponse> {
  controller::get(db, auth, org_id, cred_id).await
}

#[utoipa::path(
  get,
  operation_id = "Get All Aws Credentials",
  path = "/organization/<org_id>/credentials/aws",
  tag = "Organization Credentials Aws",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Get all Aws credentials", body = Vec<AwsCredentialsInfoResponse>),
  ),
)]
#[get("/organization/<org_id>/credentials/aws", format = "application/json")]
pub async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<AwsCredentialsInfoResponse>> {
  controller::get_all(db, auth, org_id).await
}

#[utoipa::path(
  patch,
  operation_id = "Update Aws Credential",
  path = "/organization/<org_id>/credentials/aws/<cred_id>",
  tag = "Organization Credentials Aws",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Successfully updated Aws credential", body = SuccessMessage),
  ),
  request_body = UpdateAwsCredentialsRequest,
)]
#[patch(
  "/organization/<org_id>/credentials/aws/<cred_id>",
  format = "application/json",
  data = "<body>"
)]
pub async fn update(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateAwsCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, auth, org_id, cred_id, body).await
}

#[utoipa::path(
  delete,
  operation_id = "Delete Aws Credential",
  path = "/organization/<org_id>/credentials/aws/<cred_id>",
  tag = "Organization Credentials Aws",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Successfully deleted Aws credential", body = SuccessMessage),
  ),
)]
#[delete(
  "/organization/<org_id>/credentials/aws/<cred_id>",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, auth, org_id, cred_id).await
}
