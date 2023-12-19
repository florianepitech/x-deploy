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
pub(crate) struct Message {
  #[serde(rename = "message")]
  pub(crate) message: String,
}

impl Message {
  pub(crate) fn new(message: String) -> Self {
    Self { message }
  }
}

pub type CustomResponse<T> = Result<Custom<Json<T>>, Custom<Json<Message>>>;

pub fn custom_message<T: Serialize>(
  status: rocket::http::Status,
  message: &str,
) -> CustomResponse<T> {
  let message = Message::new(message.to_string());
  Err(Custom(status, Json(message)))
}

pub fn custom_response<T: Serialize>(
  status: rocket::http::Status,
  body: T,
) -> CustomResponse<T> {
  Ok(Custom(status, Json(body)))
}
