use crate::error::ApiError;
use rocket::http::Status;

pub(crate) mod organization;
pub(crate) mod project;
pub(crate) mod user;

impl From<mongodb::error::Error> for ApiError {
  fn from(err: mongodb::error::Error) -> Self {
    error!("Error while sending request to database: {}", err);
    let message = "Error while sending request to database".to_string();
    Self::new(Status::InternalServerError, message)
  }
}
