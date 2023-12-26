use crate::cipher::password::{
  hash_password, is_strong_password, verify_password,
};
use crate::cipher::two_factor::{new_2fa, verify_2fa_code};
use crate::db::query::user::email::confirm_email;
use crate::db::query::user::get_user_from_db;
use crate::db::query::user::password::update_password;
use crate::db::query::user::two_factor::{
  delete_2fa_in_db, setup_2fa_in_db, update_2fa_state_in_db,
};
use crate::guard::token::Token;
use crate::route::account::dto;
use crate::route::account::dto::{
  ChangePasswordBody, ChangePhoneBody, GetAccountInfoResponse,
  TwoFactorCodeRequest, TwoFactorInfoRequest, TwoFactorInfoResponse,
  TwoFactorSetupRequest, TwoFactorSetupResponse, VerifyEmailBody,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResponse, SuccessMessage,
};
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) async fn get_info(
  token: Token,
  db: &State<Database>,
) -> ApiResponse<GetAccountInfoResponse> {
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;
  let response = GetAccountInfoResponse {
    firstname: user.firstname,
    lastname: user.lastname,
    email: user.email.email,
    email_verified: user.email.verified,
    phone: user.phone.phone,
  };
  custom_response(Status::Ok, response)
}

pub(crate) async fn verify_email(
  db: &State<Database>,
  token: Token,
  body: Json<VerifyEmailBody>,
) -> ApiResponse<SuccessMessage> {
  let id = token.parse_id()?;
  let user = get_user_from_db(db, &id).await?;
  // Check if email is already verified
  if user.email.verified {
    return custom_error(Status::BadRequest, "Email is already verified");
  }
  let body_code = body.code.clone();
  let user_code = match user.email.code.clone() {
    None => {
      return custom_error(
        Status::InternalServerError,
        "Code for verify email is empty in database",
      )
    }
    Some(code) => code,
  };
  // Verify if code is correct
  if body_code != user_code {
    return custom_error(
      Status::BadRequest,
      "Code for verify email is invalid",
    );
  }
  confirm_email(&db.inner(), &id).await?;
  custom_message(Status::Ok, "Your email is now verified")
}

pub(crate) async fn change_password(
  db: &State<Database>,
  token: Token,
  body: Json<ChangePasswordBody>,
) -> ApiResponse<SuccessMessage> {
  let id = token.parse_id()?;
  let user = get_user_from_db(db, &id).await?;
  // Verify password of the account
  let hash_actual_password = user.password.password.clone();
  let actual_password = body.actual_password.clone();
  let valid =
    verify_password(actual_password.as_str(), hash_actual_password.as_str())?;
  if !valid {
    return custom_error(
      Status::Unauthorized,
      "The password of your account for changing password is invalid",
    );
  }
  if is_strong_password(&body.new_password)? {
    return custom_error(
      Status::BadRequest,
      "The new password is not strong enough",
    );
  }
  let hash_new_password = hash_password(body.new_password.clone().as_str())?;
  // Update password in database
  let result = update_password(db, &id, &hash_new_password).await?;
  if result.modified_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Password is not updated in database",
    );
  }
  custom_message(Status::Ok, "Your password is now updated")
}

pub(crate) async fn change_phone(
  db: &State<Database>,
  body: Json<ChangePhoneBody>,
) -> ApiResponse<SuccessMessage> {
  let new_phone = body.new_phone.clone();
  todo!("Change phone")
}

// 2FA

pub(crate) async fn info_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorInfoRequest>,
) -> ApiResponse<TwoFactorInfoResponse> {
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;
  // Verify password of the account
  let valid =
    verify_password(body.password.as_str(), user.password.password.as_str())?;
  if (!valid) {
    return custom_error(
      Status::Unauthorized,
      "The password of your account for getting 2FA info is invalid",
    );
  }
  // Verify 2FA is setup
  if let None = user.two_factor.clone() {
    return custom_error(
      Status::BadRequest,
      "2FA is not setup for this account",
    );
  }
  let two_factor = user.two_factor.clone().unwrap();
  let totp = crate::cipher::two_factor::from_two_factor(
    &two_factor,
    user.email.email.clone(),
  )?;
  let response = TwoFactorInfoResponse {
    secret: totp.get_secret_base32(),
    enabled: two_factor.is_enabled(),
    qr_code: totp.get_qr_base64().unwrap(),
  };
  return custom_response(Status::Ok, response);
}

pub(crate) async fn setup_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorSetupRequest>,
) -> ApiResponse<TwoFactorSetupResponse> {
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;
  let valid_password =
    verify_password(body.password.as_str(), user.password.password.as_str())?;
  if !valid_password {
    return custom_error(
      Status::Unauthorized,
      "The password provided for setup 2FA is invalid",
    );
  }
  if let Some(two_factor) = user.two_factor.clone() {
    // Verify 2FA is not already enabled
    return match two_factor.is_enabled() {
      true => custom_error(
        Status::BadRequest,
        "2FA is already enabled for this account",
      ),
      false => {
        // 2FA is already generated, return the secret
        let totp = crate::cipher::two_factor::from_two_factor(
          &two_factor,
          user.email.email.clone(),
        )?;
        let response: TwoFactorSetupResponse = TwoFactorSetupResponse {
          recovery_code: two_factor.recovery_code,
          qr_code: totp.get_qr_base64().unwrap(),
        };
        return custom_response(Status::Ok, response);
      }
    };
  }
  // Setup the 2FA in database
  let new_two_factor = new_2fa(user.email.email.clone())?;
  let two_factor = setup_2fa_in_db(db, &user, &new_two_factor).await?;
  // Return the 2FA secret
  let response: TwoFactorSetupResponse = TwoFactorSetupResponse {
    recovery_code: two_factor.recovery_code,
    qr_code: new_two_factor.get_qr_base64().unwrap(),
  };
  custom_response(Status::Ok, response)
}

pub(crate) async fn enable_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;
  // Verify 2FA is setup
  if let None = user.two_factor.clone() {
    return custom_message(
      Status::BadRequest,
      "2FA is not setup for this account, please setup it first",
    );
  }
  // Verify 2FA is not already enabled
  let two_factor = user.two_factor.clone().unwrap();
  if two_factor.is_enabled() {
    return custom_message(
      Status::BadRequest,
      "2FA is already enabled for this account",
    );
  }
  // Verify 2FA code
  let valid =
    verify_2fa_code(user.email.email.clone(), &two_factor, body.code.clone())?;
  if !valid {
    return custom_message(
      Status::BadRequest,
      "2FA code is invalid for verifying",
    );
  }
  // Update 2FA state in database
  update_2fa_state_in_db(db, &user, true).await?;
  custom_message(Status::Ok, "Your 2FA is now enabled")
}

pub(crate) async fn disable_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;
  // Verify 2FA is setup
  if let None = user.two_factor.clone() {
    return custom_message(
      Status::BadRequest,
      "2FA is not setup for this account",
    );
  }
  // Verify 2FA is not already disabled
  let two_factor = user.two_factor.clone().unwrap();
  if !two_factor.is_enabled() {
    return custom_message(
      Status::BadRequest,
      "2FA is already setup but disabled for this account",
    );
  }
  // Verify 2FA code
  verify_2fa_code(user.email.email.clone(), &two_factor, body.code.clone())?;
  // Update 2FA state in database
  delete_2fa_in_db(db, &user_id).await?;
  custom_message(Status::Ok, "Your 2FA is now disabled")
}
