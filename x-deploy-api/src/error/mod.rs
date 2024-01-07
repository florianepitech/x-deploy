use crate::route::SuccessMessage;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct ApiError {
  pub status: Status,
  pub message: String,
}

impl ApiError {
  pub(crate) fn new(
    status: Status,
    message: String,
  ) -> Self {
    ApiError { status, message }
  }
}

impl Debug for ApiError {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "{}", self.message)
  }
}

impl Display for ApiError {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "{}", self.message)
  }
}

impl Error for ApiError {}
