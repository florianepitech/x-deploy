use crate::cipher::password::{
  hash_password, is_strong_password, verify_password,
};
use crate::cipher::two_factor::{
  generate_recovery_code, new_2fa, verify_2fa_code,
};
use crate::guard::token::Token;
use crate::route::account::dto::{
  ChangePasswordRequest, ChangePhoneRequest, GetAccountInfoResponse,
  TwoFactorCodeRequest, TwoFactorInfoRequest, TwoFactorInfoResponse,
  TwoFactorSetupRequest, TwoFactorSetupResponse, VerifyEmailRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use x_deploy_common::db::query::user::email::confirm_email;
use x_deploy_common::db::query::user::password::query_user_password_update_hash;
use x_deploy_common::db::user::{TwoFactor, User};

pub(crate) async fn get_info(
  token: Token,
  db: &State<Database>,
) -> ApiResult<GetAccountInfoResponse> {
  let user_id = token.parse_id()?;
  let user = match User::find_with_id(db, &user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
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
  body: Json<VerifyEmailRequest>,
) -> ApiResult<SuccessMessage> {
  let id = token.parse_id()?;
  let user = match User::find_with_id(db, &id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
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
  body: Json<ChangePasswordRequest>,
) -> ApiResult<SuccessMessage> {
  let id = token.parse_id()?;
  let user = match User::find_with_id(db, &id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
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
  let result =
    query_user_password_update_hash(db, &id, &hash_new_password).await?;
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
  body: Json<ChangePhoneRequest>,
) -> ApiResult<SuccessMessage> {
  let new_phone = body.new_phone.clone();
  todo!("Change phone")
}

// 2FA

pub(crate) async fn info_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorInfoRequest>,
) -> ApiResult<TwoFactorInfoResponse> {
  let user_id = token.parse_id()?;
  let user = match User::find_with_id(db, &user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
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
) -> ApiResult<TwoFactorSetupResponse> {
  let user_id = token.parse_id()?;
  let mut user = match User::find_with_id(db, &user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
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
  let recovery_code = generate_recovery_code();
  user.two_factor = Some(TwoFactor {
    setup: None,
    recovery_code: recovery_code.clone(),
    secret_base32: new_two_factor.get_secret_base32(),
  });
  let update = user.two_factor_update(db).await?;
  if update.modified_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Failed to update 2FA in database",
    );
  }
  // Return the 2FA secret
  let response: TwoFactorSetupResponse = TwoFactorSetupResponse {
    recovery_code,
    qr_code: new_two_factor.get_qr_base64().unwrap(),
  };
  custom_response(Status::Ok, response)
}

pub(crate) async fn enable_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let mut user = match User::find_with_id(db, &user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  // Verify 2FA is setup
  if let None = user.two_factor.clone() {
    return custom_message(
      Status::BadRequest,
      "2FA is not setup for this account, please setup it first",
    );
  }
  // Verify 2FA is not already enabled
  let mut two_factor = user.two_factor.clone().unwrap();
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
  two_factor.setup = Some(bson::DateTime::now());
  user.two_factor = Some(two_factor);
  let update = user.two_factor_update(db).await?;
  if update.modified_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Failed to update 2FA in database",
    );
  }
  custom_message(Status::Ok, "Your 2FA is now enabled")
}

pub(crate) async fn disable_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let mut user = match User::find_with_id(db, &user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
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
  user.two_factor = None;
  let update = user.two_factor_update(db).await?;
  if update.modified_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Failed to update 2FA in database",
    );
  }
  custom_message(Status::Ok, "Your 2FA is now disabled")
}
