use crate::guard::bearer_token::BearerToken;
use crate::route::organization::role::dto::CustomRoleInfoResponse;
use crate::route::ApiResult;
use mongodb::Database;
use rocket::State;

mod controller;
mod dto;

#[utoipa::path(
    get,
    operation_id = "Get All Roles",
    path = "/organization/<org_id>/role",
    tag = "Organization Role",
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
