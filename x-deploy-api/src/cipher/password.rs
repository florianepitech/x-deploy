use crate::error::ApiError;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;

pub(crate) fn hash_password(password: &str) -> Result<String, ApiError> {
  let result = hash(password, DEFAULT_COST);
  return match result {
    Ok(result) => Ok(result),
    Err(_) => {
      let message = "Error while hashing password".to_string();
      Err(ApiError::new(Status::InternalServerError, message))
    }
  };
}

pub(crate) fn verify_password(
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

pub(crate) fn is_strong_password(password: &String) -> Result<bool, ApiError> {
  let regex = r"^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*#?&])[A-Za-z\d@$!%*#?&]{8,}$";
  let result = regex::Regex::new(regex);
  match result {
    Ok(result) => {
      let result = result.is_match(password);
      Ok(result)
    }
    Err(_) => {
      let message = "Error while verifying password".to_string();
      return Err(ApiError::new(Status::InternalServerError, message));
    }
  }
}
