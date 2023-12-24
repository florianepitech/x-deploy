use crate::error::ApiError;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub(crate) mod deploy;

pub mod account;
pub mod auth;
pub mod organization;
pub mod ovh;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub(crate) struct SuccessMessage {
  #[serde(rename = "message")]
  pub(crate) message: String,
}

impl SuccessMessage {
  pub(crate) fn new(message: String) -> Self {
    Self { message }
  }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub(crate) struct ErrorMessage {
  #[serde(rename = "error")]
  pub(crate) error: String,
}

impl ErrorMessage {
  pub(crate) fn new(error: String) -> Self {
    Self { error }
  }
}

impl From<ApiError> for Custom<Json<ErrorMessage>> {
  fn from(error: ApiError) -> Self {
    let status = error.status;
    let error = ErrorMessage::new(error.to_string());
    Custom(status, Json(error))
  }
}

pub type ApiResponse<T> = Result<Custom<Json<T>>, Custom<Json<ErrorMessage>>>;

pub fn custom_error<T: Serialize>(
  status: rocket::http::Status,
  error: &str,
) -> ApiResponse<T> {
  let error = ErrorMessage::new(error.to_string());
  Err(Custom(status, Json::<ErrorMessage>(error)))
}

pub fn custom_message(
  status: rocket::http::Status,
  message: &str,
) -> ApiResponse<SuccessMessage> {
  let message = SuccessMessage::new(message.to_string());
  Ok(Custom(status, Json::<SuccessMessage>(message)))
}

pub fn custom_response<T: Serialize>(
  status: rocket::http::Status,
  body: T,
) -> ApiResponse<T> {
  Ok(Custom(status, Json(body)))
}
