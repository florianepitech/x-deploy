use crate::guard::token::Token;
use crate::route::organization::api_key::dto::CreateApiKeyRequest;
use crate::route::{custom_message, ApiResult, SuccessMessage};
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<CreateApiKeyRequest>,
) -> ApiResult<SuccessMessage> {
  let objectId = ObjectId::from_str(&id);
  if objectId.is_err() {
    return custom_message(Status::BadRequest, "Invalid organization id");
  }
  let organization_id = objectId.unwrap();
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResult<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: String,
  key_id: String,
) -> ApiResult<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  key_id: String,
) -> ApiResult<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}
