use crate::db::user::TwoFactor;
use crate::route::Message;
use crate::DOTENV_CONFIG;
use k8s_openapi::chrono;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use totp_rs::{Algorithm, Secret, TOTP};

const DIGITS: usize = 6;
const SKEW: u8 = 1;
const STEP: u64 = 30;

pub(crate) fn new_2fa(email: String) -> Result<TOTP, Custom<Json<Message>>> {
  let secret = Secret::default().to_bytes().unwrap();
  let app_name = DOTENV_CONFIG.app_name.clone();
  let result = totp_rs::TOTP::new(
    Algorithm::SHA256,
    DIGITS,
    SKEW,
    STEP,
    secret,
    Some(app_name),
    email,
  );
  match result {
    Ok(totp) => Ok(totp),
    Err(e) => Err(Custom(
      Status::InternalServerError,
      Json(Message {
        message: format!("Error while creating 2FA: {}", e),
      }),
    )),
  }
}

pub(crate) fn from_two_factor(
  two_factor: &TwoFactor,
  email: String,
) -> Result<TOTP, Custom<Json<Message>>> {
  let app_name = DOTENV_CONFIG.app_name.clone();
  let result = totp_rs::TOTP::new(
    Algorithm::SHA256,
    DIGITS,
    SKEW,
    STEP,
    two_factor.secret.clone(),
    Some(app_name),
    email,
  );
  match result {
    Ok(totp) => Ok(totp),
    Err(e) => Err(Custom(
      Status::InternalServerError,
      Json(Message {
        message: format!("Error while creating 2FA: {}", e),
      }),
    )),
  }
}

pub(crate) fn verify_2fa_code(
  email: String,
  two_factor: TwoFactor,
  code: String,
) -> Result<(), Custom<Json<Message>>> {
  let totp = from_two_factor(&two_factor, email)?;
  let check = totp.check_current(code.as_str());
  match check {
    Ok(result) => {
      if result {
        Ok(())
      } else {
        Err(Custom(
          Status::BadRequest,
          Json(Message {
            message: "Invalid 2FA code".to_string(),
          }),
        ))
      }
    }
    Err(e) => Err(Custom(
      Status::InternalServerError,
      Json(Message {
        message: format!("Error while creating 2FA: {}", e),
      }),
    )),
  }
}
