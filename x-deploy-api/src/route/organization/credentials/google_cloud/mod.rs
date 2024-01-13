use crate::guard::bearer_token::BearerToken;
use crate::route::{custom_message, ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::http::Status;
use rocket::State;

#[deprecated]
#[post(
  "/organization/<id>/credentials/google-cloud",
  format = "application/json"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  id: String,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[deprecated]
#[get(
  "/organization/<id>/credentials/google-cloud",
  format = "application/json"
)]
pub(crate) async fn get(
  db: &State<Database>,
  token: BearerToken,
  id: String,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

#[deprecated]
#[delete(
  "/organization/<id>/credentials/google-cloud",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  id: String,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
