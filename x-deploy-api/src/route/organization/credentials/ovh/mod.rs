mod controller;
pub mod dto;

use crate::guard::auth::Auth;
use crate::route::{ApiResult, SuccessMessage};
use dto::{
  NewOvhCredentialsRequest, OvhCredentialsInfoResponse,
  UpdateOvhCredentialsRequest,
};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

#[utoipa::path(
  post,
  operation_id = "Create Ovh Credential",
  path = "/organization/<org_id>/credentials/ovh",
  tag = "Organization Credentials Ovh",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 201, description = "Successfully created new Ovh credential", body = SuccessMessage),
  ),
  request_body = NewOvhCredentialsRequest,
)]
#[post(
  "/organization/<org_id>/credentials/ovh",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  body: Json<NewOvhCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, auth, org_id, body).await
}

#[utoipa::path(
  get,
  operation_id = "Get Ovh Credentials",
  path = "/organization/<org_id>/credentials/ovh/<cred_id>",
  tag = "Organization Credentials Ovh",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Get Ovh credential", body = OvhCredentialsInfoResponse),
  ),
)]
#[get(
  "/organization/<org_id>/credentials/ovh/<cred_id>",
  format = "application/json"
)]
pub(crate) async fn get(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<OvhCredentialsInfoResponse> {
  controller::get(db, auth, org_id, cred_id).await
}

#[utoipa::path(
  get,
  operation_id = "Get All Ovh Credentials",
  path = "/organization/<org_id>/credentials/ovh",
  tag = "Organization Credentials Ovh",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Get all Ovh credentials", body = Vec<OvhCredentialsInfoResponse>),
  ),
)]
#[get("/organization/<org_id>/credentials/ovh", format = "application/json")]
pub async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<OvhCredentialsInfoResponse>> {
  controller::get_all(db, auth, org_id).await
}

#[utoipa::path(
  patch,
  operation_id = "Update Ovh Credential",
  path = "/organization/<org_id>/credentials/ovh/<cred_id>",
  tag = "Organization Credentials Ovh",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Successfully updated Ovh credential", body = SuccessMessage),
  ),
  request_body = UpdateOvhCredentialsRequest,
)]
#[patch(
  "/organization/<org_id>/credentials/ovh/<cred_id>",
  format = "application/json",
  data = "<body>"
)]
pub async fn update(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateOvhCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, auth, org_id, cred_id, body).await
}

#[utoipa::path(
  delete,
  operation_id = "Delete Ovh Credential",
  path = "/organization/<org_id>/credentials/ovh/<cred_id>",
  tag = "Organization Credentials Ovh",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "Successfully deleted Ovh credential", body = SuccessMessage),
  ),
)]
#[delete(
  "/organization/<org_id>/credentials/ovh/<cred_id>",
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
