use rocket::http::Status;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use x_deploy_common::CommonError;

#[derive(Debug)]
pub struct ApiError {
  pub status: Status,
  pub message: String,
}

impl Display for ApiError {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "{}", self.message)
  }
}

impl ApiError {
  pub(crate) fn new(
    status: Status,
    message: String,
  ) -> Self {
    ApiError { status, message }
  }
}

impl From<CommonError> for ApiError {
  fn from(e: CommonError) -> Self {
    let status = Status::InternalServerError;
    return match e {
      CommonError::FromStrError(message) => ApiError::new(status, message),
      _ => ApiError::new(
        status,
        "An internal error occurred, please try again later".to_string(),
      ),
    };
  }
}

impl From<bson::oid::Error> for ApiError {
  fn from(_: bson::oid::Error) -> Self {
    let status = Status::InternalServerError;
    let error = ApiError::new(
      status,
      "Fail to parse your id, please try again later".to_string(),
    );
    error
  }
}

impl From<chrono::ParseError> for ApiError {
  fn from(_: chrono::ParseError) -> Self {
    let status = Status::InternalServerError;
    let error = ApiError::new(
      status,
      "Fail to parse your date, please try with other format".to_string(),
    );
    error
  }
}

impl From<validator::ValidationErrors> for ApiError {
  fn from(e: validator::ValidationErrors) -> Self {
    let status = Status::BadRequest;
    let message = e.to_string();
    ApiError::new(status, message)
  }
}

impl From<reqwest::Error> for ApiError {
  fn from(error: reqwest::Error) -> Self {
    ApiError::new(
      Status::InternalServerError,
      format!("Could not send request to external service: {}", error),
    )
  }
}

impl From<serde_json::Error> for ApiError {
  fn from(error: serde_json::Error) -> Self {
    ApiError::new(
      Status::InternalServerError,
      format!("Could not parse response from external service: {}", error),
    )
  }
}

impl Error for ApiError {}
