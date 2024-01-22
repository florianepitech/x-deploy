use crate::guard::bearer_token::BearerToken;
use crate::route::organization::role::dto::{
  CreateCustomRoleRequest, CustomRoleInfoResponse, UpdateCustomRoleRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub mod dto;

#[utoipa::path(
    get,
    operation_id = "Get All Roles",
    path = "/organization/<org_id>/role",
    tag = "Organization Role",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Get all custom roles", body = Vec<CustomRoleInfoResponse>),
    ),
)]
#[get("/organization/<org_id>/role", format = "application/json")]
pub(crate) async fn all(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<CustomRoleInfoResponse>> {
  controller::all(db, token, org_id).await
}

// TODO: Add method to modify permission of a role

#[utoipa::path(
    patch,
    operation_id = "Update a role by id",
    path = "/organization/<org_id>/role/<role_id>",
    tag = "Organization Role",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Update a custom role", body = SuccessMessage),
    ),
    request_body = UpdateCustomRoleRequest,
)]
#[patch(
  "/organization/<org_id>/role/<role_id>",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  role_id: &str,
  body: Json<UpdateCustomRoleRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, token, org_id, role_id, body).await
}

#[utoipa::path(
    post,
    operation_id = "Create Role",
    path = "/organization/<org_id>/role",
    tag = "Organization Role",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Create a new custom role", body = SuccessMessage),
    ),
    request_body = CreateCustomRoleRequest,
)]
#[post(
  "/organization/<org_id>/role",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  body: Json<CreateCustomRoleRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, token, org_id, body).await
}

#[utoipa::path(
    delete,
    operation_id = "Delete Role",
    path = "/organization/<org_id>/role/<role_id>",
    tag = "Organization Role",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Delete a custom role", body = SuccessMessage),
    ),
)]
#[delete("/organization/<org_id>/role/<role_id>", format = "application/json")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  role_id: &str,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, token, org_id, role_id).await
}

#[utoipa::path(
    get,
    operation_id = "Get Role By Id",
    path = "/organization/<org_id>/role/<role_id>",
    tag = "Organization Role",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Get a custom role by id", body = CustomRoleInfoResponse),
    ),
)]
#[get("/organization/<org_id>/role/<role_id>", format = "application/json")]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  role_id: &str,
) -> ApiResult<CustomRoleInfoResponse> {
  controller::get_by_id(db, token, org_id, role_id).await
}
