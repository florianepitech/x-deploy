use crate::guard::token::Token;
use crate::route::{custom_message, CustomResponse, Message};
use bson::{doc, oid};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

#[get("/organization/<id>/member", format = "application/json")]
pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> CustomResponse<Message> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[delete("/organization/<id>/member/<member_id>", format = "application/json")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  member_id: String,
) -> CustomResponse<Message> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
