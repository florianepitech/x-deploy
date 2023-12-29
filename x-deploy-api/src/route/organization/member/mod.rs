mod controller;
mod dto;

use crate::guard::token::Token;
use crate::route::organization::member::dto::MemberInfoResponse;
use crate::route::{ApiResponse, SuccessMessage};
use bson::doc;
use mongodb::Database;
use rocket::State;

#[utoipa::path(
  post,
  path = "/organization/<org_id>/member",
  tag = "Organization Members"
)]
#[get("/organization/<org_id>/member", format = "application/json")]
pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResponse<Vec<MemberInfoResponse>> {
  controller::get_all(db, token, org_id).await
}

#[utoipa::path(
    post,
    path = "/organization/<org_id>/member",
    tag = "Organization Members",
    responses(
        (status = 200, description = "Member was removed from organization", body = SuccessMessage),
    )
)]
#[delete(
  "/organization/<org_id>/member/<member_id>",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  org_id: String,
  member_id: String,
) -> ApiResponse<SuccessMessage> {
  controller::delete(db, token, org_id, member_id).await
}
