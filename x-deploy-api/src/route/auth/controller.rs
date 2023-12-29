use crate::cipher::password::{
  generate_forgot_password_token, hash_password, is_strong_password,
  verify_password,
};
use crate::cipher::two_factor::verify_2fa_code;
use crate::db::query::user::password::{
  query_user_password_from_token, query_user_password_update_hash,
  query_user_password_update_token,
};
use crate::db::query::user::two_factor::delete_2fa_in_db;
use crate::db::query::user::{get_user_from_db, get_user_from_email};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::event::user::{
  send_forgot_password_event, send_magic_link_event, send_password_reset_event,
  send_user_registered_event,
};
use crate::guard::token::Token;
use crate::route::auth::controller;
use crate::route::auth::dto::{
  ForgotPasswordRequest, LoginRequest, LoginResponse, MagicLinkRequest,
  RegisterRequest, ResetPasswordRequest, TwoFactorCode,
  TwoFactorRecoveryRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResponse, SuccessMessage,
};
use bson::doc;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) async fn login(
  db: &State<Database>,
  body: Json<LoginRequest>,
) -> ApiResponse<LoginResponse> {
  let login_body = body.into_inner();
  // Verify if email exists for an user
  let user = get_user_from_email(db, &login_body.email).await?;
  // Verify if password is correct
  let valid_password =
    verify_password(&login_body.password, user.password.password.as_str())?;
  if !valid_password {
    return custom_error(
      Status::Unauthorized,
      "Email or password is incorrect",
    );
  }
  let two_factor: Option<bool> = if let None = user.two_factor.clone() {
    None
  } else {
    Some(false)
  };
  let token = Token::new(user.id.clone(), two_factor)?;
  let jwt = token.to_jwt()?;
  let response = LoginResponse { token: jwt };
  custom_response(Status::Ok, response)
}

pub(crate) async fn magic_link(
  db: &State<Database>,
  body: Json<MagicLinkRequest>,
) -> ApiResponse<SuccessMessage> {
  let email = body.email.clone();
  let user = get_user_from_email(db, &email).await?;
  let two_factor: Option<bool> = if let None = user.two_factor.clone() {
    None
  } else {
    Some(false)
  };
  let token = Token::new(user.id.clone(), two_factor)?;
  let jwt = token.to_jwt()?;
  let _ = send_magic_link_event(user.id, email, jwt);
  custom_message(Status::Ok, "You will receive a magic link in your email")
}

pub(crate) async fn register(
  db: &State<Database>,
  body: Json<RegisterRequest>,
) -> ApiResponse<SuccessMessage> {
  let body = body.into_inner();
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  // Verify if email exists for an user
  let user = collection
    .find_one(
      doc! {
          "email.email": body.email.clone()
      },
      None,
    )
    .await
    .unwrap();
  if user.is_some() {
    return custom_error(Status::Conflict, "Email already exists for an user");
  }
  // Verify if password is strong
  let strong = is_strong_password(&body.password.clone())?;
  if !strong {
    return custom_error(
      Status::BadRequest,
      "Password is not strong enough, please use a stronger password",
    );
  }
  let password_hash =
    crate::cipher::password::hash_password(body.password.as_str())?;
  let new_user: User = User::new(
    body.firstname.clone(),
    body.lastname.clone(),
    password_hash,
    body.email.clone(),
    body.password.clone(),
  );
  let id = new_user.id.clone();
  collection.insert_one(new_user, None).await.unwrap();
  let _ =
    send_user_registered_event(id, body.firstname, body.lastname, body.email);
  return custom_message(Status::Created, "You are now registered");
}

pub(crate) async fn two_factor(
  db: &State<Database>,
  body: Json<TwoFactorCode>,
) -> ApiResponse<LoginResponse> {
  let mut token = Token::parse_jwt(&body.token)?;
  if token.is_expired() {
    return custom_error(Status::Unauthorized, "Token is expired");
  }
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;

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
) -> ApiResponse<LoginResponse> {
  let mut token = Token::parse_jwt(&body.token)?;
  if token.is_expired() {
    return custom_error(Status::Unauthorized, "Token is expired");
  }
  let user_id = token.parse_id()?;
  let user = get_user_from_db(db, &user_id).await?;
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
      delete_2fa_in_db(db, &user_id).await?;
      // Generate a new token with jwt ans send the jwt
      let token = Token::new(user_id, None)?;
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
) -> ApiResponse<SuccessMessage> {
  let email = body.email.clone();
  let user = get_user_from_email(db, &email).await?;
  let token = generate_forgot_password_token();
  query_user_password_update_token(db, &user.id, Some(&token)).await?;
  let _ = send_forgot_password_event(
    user.id,
    user.firstname,
    user.lastname,
    user.email.email,
    token,
  );
  custom_message(
    Status::Ok,
    "You will receive a link to reset your password in your email",
  )
}

pub(crate) async fn reset_password(
  db: &State<Database>,
  body: Json<ResetPasswordRequest>,
) -> ApiResponse<SuccessMessage> {
  let user = query_user_password_from_token(db, &body.token).await?;
  return match user {
    Some(user) => {
      let strong = is_strong_password(&body.new_password)?;
      if !strong {
        return custom_error(
          Status::BadRequest,
          "Password is not strong enough, please use a stronger password",
        );
      }
      let password_hash = hash_password(body.new_password.as_str())?;
      let _ = query_user_password_update_token(db, &user.id, None).await?;
      let _ =
        query_user_password_update_hash(db, &user.id, &password_hash).await?;
      let _ = send_password_reset_event(
        user.id,
        user.firstname,
        user.lastname,
        user.email.email,
      );
      custom_message(Status::Ok, "Your password was reset")
    }
    None => custom_error(Status::Unauthorized, "Token is invalid"),
  };
}
