use crate::guard::token::Token;
use crate::route::organization::custom_role::dto::CustomRoleInfoResponse;
use crate::route::ApiResponse;
use mongodb::Database;
use rocket::State;

mod controller;
mod dto;

#[utoipa::path(
    get,
    path = "/organization/<org_id>/custom_role",
    tag = "Organization Custom Role",
    responses(
        (status = 200, description = "Get all custom roles", body = Vec<CustomRoleInfoResponse>),
    ),
)]
#[get("/organization/<org_id>/custom_role", format = "application/json")]
pub(crate) async fn all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResponse<Vec<CustomRoleInfoResponse>> {
  controller::all(db, token, org_id).await
}
