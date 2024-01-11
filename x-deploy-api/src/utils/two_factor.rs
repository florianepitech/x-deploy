use crate::error::ApiError;
use crate::CONFIG;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::http::Status;
use totp_rs::{Algorithm, Secret, SecretParseError, TotpUrlError, TOTP};
use x_deploy_common::db::user::TwoFactor;

const DIGITS: usize = 6;
const SKEW: u8 = 1;
const STEP: u64 = 30;

const RECOVERY_CODE_LENGTH: usize = 30;

pub(crate) fn new_2fa(email: String) -> Result<TOTP, ApiError> {
  let secret = Secret::default().to_bytes().unwrap();
  let app_name = CONFIG.app_name.clone();
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
    Err(_) => Err(ApiError::new(
      Status::InternalServerError,
      "Error while creating 2FA".to_string(),
    )),
  }
}

pub(crate) fn from_two_factor(
  two_factor: &TwoFactor,
  email: String,
) -> Result<TOTP, ApiError> {
  let app_name = CONFIG.app_name.clone();
  let secret_base32 = two_factor.secret_base32.clone();
  let secret = Secret::Encoded(secret_base32).to_bytes()?;
  Ok(totp_rs::TOTP::new(
    Algorithm::SHA256,
    DIGITS,
    SKEW,
    STEP,
    secret,
    Some(app_name),
    email,
  )?)
}

pub(crate) fn verify_2fa_code(
  email: String,
  two_factor: &TwoFactor,
  code: String,
) -> Result<bool, ApiError> {
  let totp = from_two_factor(&two_factor, email)?;
  let check = totp.check_current(code.as_str());
  match check {
    Ok(result) => Ok(result),
    Err(_) => Err(ApiError::new(
      Status::InternalServerError,
      "Error while verifying 2FA code".to_string(),
    )),
  }
}

pub(crate) fn generate_recovery_code() -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(RECOVERY_CODE_LENGTH)
    .map(char::from)
    .collect::<String>()
    .to_uppercase()
}

impl From<TotpUrlError> for ApiError {
  fn from(_: TotpUrlError) -> Self {
    ApiError::new(
      Status::InternalServerError,
      "Error with your 2FA configuration".to_string(),
    )
  }
}

impl From<SecretParseError> for ApiError {
  fn from(_: SecretParseError) -> Self {
    ApiError::new(
      Status::InternalServerError,
      "Error with your 2FA configuration".to_string(),
    )
  }
}
