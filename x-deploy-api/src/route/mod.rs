use crate::error::ApiError;
use rocket::response::status::Custom;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use utoipa::ToSchema;
use x_deploy_common::CommonError;

pub mod account;
pub mod auth;
pub mod cloud_provider;
pub mod invitation;
pub mod organization;

pub type ApiResult<T> = Result<Custom<Json<T>>, ApiError>;

// Success Message

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub(crate) struct SuccessMessage {
  #[serde(rename = "message")]
  pub message: String,
}

impl SuccessMessage {
  pub fn new(message: String) -> Self {
    Self { message }
  }
}

// Error message

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ErrorMessage {
  #[serde(rename = "error")]
  pub error: String,
}

impl ErrorMessage {
  pub(crate) fn new(error: String) -> Self {
    Self { error }
  }
}

impl From<CommonError> for ApiError {
  fn from(_: CommonError) -> Self {
    let status = rocket::http::Status::InternalServerError;
    let error = ApiError::new(
      status,
      "An internal error occurred, please try again later".to_string(),
    );
    error
  }
}

// Implement Responder for ApiError
impl<'r> Responder<'r, 'static> for ApiError {
  fn respond_to(
    self,
    _: &'r Request<'_>,
  ) -> response::Result<'static> {
    // Convert ApiError to a JSON response
    let error = ErrorMessage::new(self.message);
    let body = serde_json::to_string(&error).unwrap();
    Response::build()
      .status(self.status)
      .header(rocket::http::ContentType::JSON)
      .sized_body(body.len(), std::io::Cursor::new(body))
      .ok()
  }
}

pub fn custom_error<T: Serialize>(
  status: rocket::http::Status,
  error: &str,
) -> ApiResult<T> {
  let error = ErrorMessage::new(error.to_string());
  Err(ApiError::new(status, error.error))
}

pub fn custom_message(
  status: rocket::http::Status,
  message: &str,
) -> ApiResult<SuccessMessage> {
  let message = SuccessMessage::new(message.to_string());
  Ok(Custom(status, Json::<SuccessMessage>(message)))
}

pub fn custom_response<T: Serialize>(
  status: rocket::http::Status,
  body: T,
) -> ApiResult<T> {
  Ok(Custom(status, Json(body)))
}
