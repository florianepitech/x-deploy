use crate::guard::token::Token;
use crate::route::organization::invitation::dto::InvitationInfoResponse;
use crate::route::{ApiResponse, SuccessMessage};
use mongodb::Database;
use rocket::State;

mod controller;
mod dto;

#[utoipa::path(
    post,
    path = "/organization/<org_id>/invitation",
    tag = "Organization Invitations",
    responses(
        (status = 200, description = "List of your current invitation", body = Vec<InvitationInfoResponse>),
    )
)]
#[get("/organization/<org_id>/invitation", format = "application/json")]
pub async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: String,
) -> ApiResponse<Vec<InvitationInfoResponse>> {
  controller::get_all(db, token, org_id).await
}
