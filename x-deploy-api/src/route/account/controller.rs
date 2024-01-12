use crate::guard::token::Token;
use crate::route::account::dto::{
  ChangePasswordRequest, ChangePhoneRequest, GetAccountInfoResponse,
  TwoFactorCodeRequest, TwoFactorInfoRequest, TwoFactorInfoResponse,
  TwoFactorSetupRequest, TwoFactorSetupResponse, VerifyEmailRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::password::{
  hash_password, is_strong_password, verify_password,
};
use crate::utils::profile_picture::ProfilePicture;
use crate::utils::two_factor::{
  generate_recovery_code, new_2fa, verify_2fa_code,
};
use crate::CONFIG;
use mongodb::Database;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::{Data, State};
use x_deploy_common::db::user::{TwoFactor, User};
use x_deploy_common::db::CommonCollection;
use x_deploy_common::s3::bucket::CommonS3Bucket;
use x_deploy_common::s3::config::CommonS3Config;
use x_deploy_common::s3::file_type::CommonS3BucketType;
use CommonS3BucketType::UserProfilePicture;

pub(crate) async fn get_info(
  token: Token,
  db: &State<Database>,
) -> ApiResult<GetAccountInfoResponse> {
  let user_id = token.parse_id()?;
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  let response = GetAccountInfoResponse {
    firstname: user.firstname,
    lastname: user.lastname,
    profile_picture_url: user.profile_picture_url,
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
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&id).await? {
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
  user_collection.email_confirm(&id).await?;
  custom_message(Status::Ok, "Your email is now verified")
}

pub(crate) async fn change_password(
  db: &State<Database>,
  token: Token,
  body: Json<ChangePasswordRequest>,
) -> ApiResult<SuccessMessage> {
  let id = token.parse_id()?;
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&id).await? {
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
  let result = user_collection
    .password_update_hash(&id, &hash_new_password)
    .await?;
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
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&user_id).await? {
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
  let totp = crate::utils::two_factor::from_two_factor(
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
  let user_collection = CommonCollection::<User>::new(db);
  let mut user = match user_collection.get_by_id(&user_id).await? {
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
        let totp = crate::utils::two_factor::from_two_factor(
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
  let update = user_collection
    .two_factor_update(&user_id, &user.two_factor)
    .await?;
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
  let user_collection = CommonCollection::<User>::new(db);
  let mut user = match user_collection.get_by_id(&user_id).await? {
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
  let update = user_collection
    .two_factor_update(&user_id, &user.two_factor)
    .await?;
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
  let user_collection = CommonCollection::<User>::new(db);
  let mut user = match user_collection.get_by_id(&user_id).await? {
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
  let update = user_collection
    .two_factor_update(&user_id, &user.two_factor)
    .await?;
  if update.modified_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Failed to update 2FA in database",
    );
  }
  custom_message(Status::Ok, "Your 2FA is now disabled")
}

pub(crate) async fn upload_profile_picture(
  db: &State<Database>,
  content_type: &ContentType,
  token: Token,
  data: Data<'_>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let user_collection = CommonCollection::<User>::new(db);
  match user_collection.get_by_id(&user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  let profile_picture = ProfilePicture::from_data(data).await?;
  let profile_picture = profile_picture.to_square()?;
  let s3_config = CommonS3Config::new(
    CONFIG.s3_endpoint.clone(),
    CONFIG.s3_bucket.clone(),
    CONFIG.s3_access_key.clone(),
    CONFIG.s3_secret_key.clone(),
    CONFIG.s3_region.clone(),
  );
  let extension = profile_picture.get_extension()?;
  let filename = format!("{}.{}", user_id, extension);
  let bytes = profile_picture.get_image_bytes()?;
  // Save file in S3
  let s3 = CommonS3Bucket::new(UserProfilePicture, s3_config);
  let content_type_str = content_type.to_string();
  s3.add(&filename, bytes.as_slice(), content_type_str)
    .await?;
  // Update profile public url
  let url = s3.get_public_url(&filename);
  user_collection
    .update_profile_picture_url(&user_id, &url)
    .await?;
  custom_message(Status::Ok, "Your profile picture is now updated")
}
