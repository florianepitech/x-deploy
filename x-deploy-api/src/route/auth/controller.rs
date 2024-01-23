use crate::guard::bearer_token::BearerToken;
use crate::oauth::{OAuth, OAuthService, OAuthUser};
use crate::route::auth::dto::{
  ForgotPasswordRequest, LoginOAuthRequest, LoginRequest, LoginResponse,
  MagicLinkRequest, RegisterRequest, ResetPasswordRequest,
  TwoFactorCodeRequest, TwoFactorRecoveryRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::password::{
  generate_forgot_password_token, hash_password, is_strong_password,
  verify_password,
};
use crate::utils::two_factor::verify_2fa_code;
use crate::CONFIG;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{tokio, State};
use std::str::FromStr;
use validator::Validate;
use x_deploy_common::db::user::User;
use x_deploy_common::db::CommonCollection;
use x_deploy_common::event::user::{
  UserForgotPasswordEvent, UserMagicLinkEvent, UserPasswordResetEvent,
  UserRegisteredEvent,
};
use x_deploy_common::event::CommonEvent;

pub(crate) async fn login(
  db: &State<Database>,
  body: Json<LoginRequest>,
) -> ApiResult<LoginResponse> {
  body.validate()?;
  tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  // Verify if email exists for an user
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.find_with_email(&body.email).await? {
    Some(user) => user,
    None => {
      return custom_error(Status::NotFound, "User not found with this email")
    }
  };
  // Verify if password is correct
  let valid_password =
    verify_password(&body.password, user.password.password.as_str())?;
  if !valid_password {
    return custom_error(
      Status::Unauthorized,
      "Email or password is incorrect for this email",
    );
  }
  let two_factor: Option<bool> = if let None = user.two_factor.clone() {
    None
  } else {
    Some(false)
  };
  let token = BearerToken::new(user.id.clone(), two_factor)?;
  let jwt = token.to_jwt()?;
  let response = LoginResponse { token: jwt };
  custom_response(Status::Ok, response)
}

pub(crate) async fn login_oauth(
  db: &State<Database>,
  body: Json<LoginOAuthRequest>,
) -> ApiResult<LoginResponse> {
  let body = body.into_inner();
  let service: OAuthService = body.service.into();
  let result: OAuthUser = OAuth::get_user(service, body.access_token).await?;
  let collection = CommonCollection::<User>::new(db);
  let user = match collection.find_with_email(&result.email).await? {
    Some(user) => user,
    None => {
      return custom_error(
        Status::NotFound,
        "User not found with this email, please register before",
      )
    }
  };
  let two_factor: Option<bool> = if let None = user.two_factor.clone() {
    None
  } else {
    Some(false)
  };
  let token = BearerToken::new(user.id.clone(), two_factor)?;
  let jwt = token.to_jwt()?;
  let response = LoginResponse { token: jwt };
  custom_response(Status::Ok, response)
}

pub(crate) async fn magic_link(
  db: &State<Database>,
  body: Json<MagicLinkRequest>,
) -> ApiResult<SuccessMessage> {
  body.validate()?;
  let email = body.email.clone();
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.find_with_email(&email).await? {
    Some(user) => user,
    None => {
      return custom_error(Status::NotFound, "User not found with this email")
    }
  };
  let two_factor: Option<bool> = if let None = user.two_factor.clone() {
    None
  } else {
    Some(false)
  };
  let token = BearerToken::new(user.id.clone(), two_factor)?;
  let jwt = token.to_jwt()?;

  CommonEvent::new(CONFIG.kafka_url.clone()).send(UserMagicLinkEvent {
    id: user.id.clone(),
    firstname: user.firstname.clone(),
    lastname: user.lastname.clone(),
    email: user.email.email.clone(),
    jwt: jwt.clone(),
  })?;
  custom_message(Status::Ok, "You will receive a magic link in your email")
}

pub(crate) async fn register(
  db: &State<Database>,
  body: Json<RegisterRequest>,
) -> ApiResult<SuccessMessage> {
  body.validate()?;
  let body = body.into_inner();
  let user_collection = CommonCollection::<User>::new(db);
  // Verify if email exists for an user
  if let Some(_) = user_collection.find_with_email(&body.email).await? {
    return custom_error(Status::Conflict, "Email already exists");
  };
  // Verify if password is strong
  let strong = is_strong_password(&body.password.clone())?;
  if !strong {
    return custom_error(
      Status::BadRequest,
      "Password is not strong enough, please use a stronger password",
    );
  }
  let password_hash = hash_password(body.password.as_str())?;
  let new_user: User = User::new(
    body.firstname.clone(),
    body.lastname.clone(),
    password_hash,
    body.email.clone(),
    body.password.clone(),
  );
  let id = new_user.id.clone();
  user_collection.insert_one(&new_user).await?;
  CommonEvent::new(CONFIG.kafka_url.clone()).send(UserRegisteredEvent {
    id: id.clone(),
    firstname: body.firstname.clone(),
    lastname: body.lastname.clone(),
    email: body.email.clone(),
  })?;
  return custom_message(Status::Created, "You are now registered");
}

pub(crate) async fn two_factor(
  db: &State<Database>,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResult<LoginResponse> {
  body.validate()?;
  let mut token = BearerToken::parse_jwt(&body.token)?;
  if token.is_expired() {
    return custom_error(Status::Unauthorized, "Token is expired");
  }
  let user_id = token.parse_id()?;
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };

  // Verify if 2 factor exist and are enabled for user
  if user.two_factor.clone().is_none() {
    return custom_error(Status::Unauthorized, "2 factor is not setup");
  }
  let user_two_factor = user.two_factor.unwrap();
  if !user_two_factor.is_enabled() {
    return custom_error(Status::Unauthorized, "2 factor is not enabled");
  }
  let result = verify_2fa_code(
    user.email.email.clone(),
    &user_two_factor,
    body.code.clone(),
  )?;
  if !result {
    return custom_error(Status::Unauthorized, "2 factor code is invalid");
  }
  token.with_otp(Some(true));
  let new_token = token.to_jwt()?;
  let response: LoginResponse = LoginResponse { token: new_token };
  return custom_response(Status::Ok, response);
}

pub(crate) async fn two_factor_recovery(
  db: &State<Database>,
  body: Json<TwoFactorRecoveryRequest>,
) -> ApiResult<LoginResponse> {
  body.validate()?;
  let mut token = BearerToken::parse_jwt(&body.token)?;
  if token.is_expired() {
    return custom_error(Status::Unauthorized, "Token is expired");
  }
  let user_id = token.parse_id()?;
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  // Verify if 2 factor exist and are enabled for user
  return match user.two_factor {
    Some(two_factor) => {
      // Verify if 2 factor is enabled
      if !two_factor.is_enabled() {
        return custom_error(
          Status::Unauthorized,
          "2 factor is not enabled for this account",
        );
      }
      // Verify if the code is valid
      let code = body.recovery_code.clone().replace(" ", "");
      let recovery_code = two_factor.recovery_code.clone();
      if !recovery_code.eq(&code) {
        return custom_error(
          Status::Unauthorized,
          "Recovery code is invalid for this account",
        );
      }
      // Disable 2 factor
      let update = user_collection.two_factor_update(&user_id, &None).await?;
      if update.modified_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Failed to update user in database",
        );
      }
      // Generate a new token with jwt ans send the jwt
      let token = BearerToken::new(user_id, None)?;
      let jwt = token.to_jwt()?;
      let response = LoginResponse { token: jwt };
      custom_response(Status::Ok, response)
    }
    None => custom_error(
      Status::Unauthorized,
      "2 factor is not setup for this account",
    ),
  };
}

pub(crate) async fn forgot_password(
  db: &State<Database>,
  body: Json<ForgotPasswordRequest>,
) -> ApiResult<SuccessMessage> {
  body.validate()?;
  let email = body.email.clone();
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.find_with_email(&email).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  // Update forgot token in database
  let token = generate_forgot_password_token();
  user_collection
    .password_update_forgot_token(&user.id, Some(&token))
    .await?;
  // Send event to kafka
  CommonEvent::new(CONFIG.kafka_url.clone()).send(UserForgotPasswordEvent {
    id: user.id.clone(),
    firstname: user.firstname.clone(),
    lastname: user.lastname.clone(),
    email: user.email.email.clone(),
    token: token.clone(),
  })?;
  custom_message(
    Status::Ok,
    "You will receive a link to reset your password in your email",
  )
}

pub(crate) async fn reset_password(
  db: &State<Database>,
  body: Json<ResetPasswordRequest>,
) -> ApiResult<SuccessMessage> {
  body.validate()?;
  // Retrieve user from database with forgot password token
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection
    .find_from_password_forgot_token(&body.token)
    .await?
  {
    Some(user) => user,
    None => return custom_error(Status::Unauthorized, "Token is invalid"),
  };
  // Verify if password is strong
  let strong = is_strong_password(&body.new_password)?;
  if !strong {
    return custom_error(
      Status::BadRequest,
      "Password is not strong enough, please use a stronger password",
    );
  }
  let password_hash = hash_password(body.new_password.as_str())?;
  // Update data in database
  user_collection
    .password_update_forgot_token(&user.id, None)
    .await?;
  user_collection
    .password_update_hash(&user.id, &password_hash)
    .await?;
  // Send event to kafka
  CommonEvent::new(CONFIG.kafka_url.clone()).send(UserPasswordResetEvent {
    id: user.id.clone(),
    firstname: user.firstname.clone(),
    lastname: user.lastname.clone(),
    email: user.email.email.clone(),
  })?;
  custom_message(Status::Ok, "Your password was reset")
}
