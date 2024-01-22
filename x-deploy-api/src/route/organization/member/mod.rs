mod controller;
pub(crate) mod dto;

use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::route::organization::member::dto::MemberInfoResponse;
use crate::route::{ApiResult, SuccessMessage};
use bson::doc;
use mongodb::Database;
use rocket::State;

#[utoipa::path(
  get,
  operation_id = "Get All Members",
  path = "/organization/<org_id>/member",
  tag = "Organization Members",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "The list of member in the organization", body = Vec<MemberInfoResponse>),
  )
)]
#[get("/organization/<org_id>/member", format = "application/json")]
pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<MemberInfoResponse>> {
  controller::get_all(db, auth, org_id).await
}

#[utoipa::path(
  get,
  operation_id = "Get a member by Id",
  path = "/organization/<org_id>/member/<member_id>",
  tag = "Organization Members",
  security(
    ("bearer" = []),
    ("apiKey" = []),
  ),
  responses(
    (status = 200, description = "The member in the organization", body = Vec<MemberInfoResponse>),
  )
)]
#[get(
  "/organization/<org_id>/member/<member_id>",
  format = "application/json"
)]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  member_id: &str,
) -> ApiResult<MemberInfoResponse> {
  controller::get_by_id(db, auth, org_id, member_id).await
}

#[utoipa::path(
    delete,
    operation_id = "Delete a member from organization",
    path = "/organization/<org_id>/member",
    tag = "Organization Members",
    security(
      ("bearer" = []),
      ("apiKey" = []),
    ),
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
  auth: Auth,
  org_id: String,
  member_id: String,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, auth, org_id, member_id).await
}
