use crate::error::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;

pub fn hash_password(password: &str) -> Result<String, ApiError> {
  let result = hash(password, DEFAULT_COST);
  return match result {
    Ok(result) => Ok(result),
    Err(_) => {
      let message = "Error while hashing password".to_string();
      Err(ApiError::new(Status::InternalServerError, message))
    }
  };
}

pub fn verify_password(
  password: &str,
  hash: &str,
) -> Result<bool, ApiError> {
  let result = verify(password, hash);
  return match result {
    Ok(result) => Ok(result),
    Err(_) => {
      let message = "Error while verifying password".to_string();
      Err(ApiError::new(Status::InternalServerError, message))
    }
  };
}
