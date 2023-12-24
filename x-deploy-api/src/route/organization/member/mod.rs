mod controller;
mod dto;

use crate::guard::token::Token;
use crate::route::{ApiResponse, SuccessMessage};
use bson::doc;
use mongodb::Database;
use rocket::State;

#[get("/organization/<id>/member", format = "application/json")]
pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  controller::get(db, token, id).await
}

#[delete("/organization/<id>/member/<member_id>", format = "application/json")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  member_id: String,
) -> ApiResponse<SuccessMessage> {
  controller::delete(db, token, id, member_id).await
}
