use crate::guard::token::Token;
use crate::route::invitation::dto::{
  InvitationInfoResponse, InvitationResponseRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub(crate) mod dto;

#[utoipa::path(
    get,
    operation_id = "Get All Invitation",
    path = "/invitation",
    tag = "Invitation",
    responses(
        (status = 200, description = "Get all invitation", body = Vec<InvitationInfoResponse>),
    ),
)]
#[get("/invitation", format = "application/json")]
pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
) -> ApiResult<Vec<InvitationInfoResponse>> {
  controller::get_all(db, token).await
}

#[utoipa::path(
    post,
    operation_id = "Response to an invitation",
    path = "/invitation/<invitation_id>",
    tag = "Invitation",
    responses(
        (status = 200, description = "Response to an invitation", body = SuccessMessage),
    ),
    request_body = InvitationResponseRequest,
)]
#[post(
  "/invitation/<invitation_id>",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn response(
  db: &State<Database>,
  token: Token,
  invitation_id: String,
  body: Json<InvitationResponseRequest>,
) -> ApiResult<SuccessMessage> {
  controller::response(db, token, invitation_id, body).await
}
