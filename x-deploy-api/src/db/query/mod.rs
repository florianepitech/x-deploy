use crate::error::ApiError;
use rocket::http::Status;

pub(crate) mod organization;
pub(crate) mod user;

impl From<mongodb::error::Error> for ApiError {
  fn from(_: mongodb::error::Error) -> Self {
    let message = "Error while sending request to database".to_string();
    Self::new(Status::InternalServerError, message)
  }
}
