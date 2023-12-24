use crate::guard::token::Token;
use crate::route::{custom_message, ApiResponse, SuccessMessage};
use mongodb::Database;
use rocket::http::Status;
use rocket::State;

pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  member_id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
