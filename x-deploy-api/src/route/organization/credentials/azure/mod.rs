use crate::guard::token::Token;
use crate::route::{custom_message, ApiResult, SuccessMessage};
use bson::oid;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

#[deprecated]
#[post("/organization/<id>/credentials/azure", format = "application/json")]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[deprecated]
#[get("/organization/<id>/credentials/azure", format = "application/json")]
pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[deprecated]
#[delete("/organization/<id>/credentials/azure", format = "application/json")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
