use crate::guard::token::Token;
use crate::route::{ApiResponse, SuccessMessage};
use mongodb::Database;
use rocket::State;

mod controller;
mod dto;

#[utoipa::path(
    post,
    path = "/organization/<id>/invitation",
    tag = "Organization Invitations",
    responses(
        (status = 200, description = "List of your current invitation", body = SuccessMessage),
    )
)]
#[post("/organization/<id>/invitation", format = "application/json")]
pub async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  controller::get(db, token, id).await
}
