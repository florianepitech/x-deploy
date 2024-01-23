use crate::route::auth::dto::{
  ForgotPasswordRequest, LoginOAuthRequest, LoginRequest, LoginResponse,
  MagicLinkRequest, RegisterRequest, ResetPasswordRequest,
  TwoFactorCodeRequest, TwoFactorRecoveryRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

mod controller;
pub mod dto;

#[utoipa::path(
    post,
    operation_id = "Login",
    path = "/auth/login/credentials",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = LoginResponse),
    ),
    request_body = LoginRequest,
)]
#[post(
  "/auth/login/credentials",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn login(
  db: &State<Database>,
  body: Json<LoginRequest>,
) -> ApiResult<LoginResponse> {
  return controller::login(db, body).await;
}

#[utoipa::path(
    post,
    operation_id = "Login OAuth",
    path = "/auth/login/oauth",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = LoginResponse),
    ),
    request_body = LoginOAuthRequest,
)]
#[post("/auth/login/oauth", format = "application/json", data = "<body>")]
pub async fn login_oauth(
  db: &State<Database>,
  body: Json<LoginOAuthRequest>,
) -> ApiResult<LoginResponse> {
  return controller::login_oauth(db, body).await;
}

#[utoipa::path(
    post,
    operation_id = "Magic Link",
    path = "/auth/magic-link",
    tag = "Auth",
    responses(
        (status = 200, description = "The magic was sent", body = SuccessMessage),
    ),
    request_body = MagicLinkRequest,
)]
#[post("/auth/magic-link", format = "application/json", data = "<body>")]
pub(crate) async fn magic_link(
  db: &State<Database>,
  body: Json<MagicLinkRequest>,
) -> ApiResult<SuccessMessage> {
  return controller::magic_link(db, body).await;
}

#[utoipa::path(
    post,
    operation_id = "Register",
    path = "/auth/register",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now registered", body = SuccessMessage)
    ),
    request_body = RegisterRequest,
)]
#[post("/auth/register", format = "application/json", data = "<body>")]
pub(crate) async fn register(
  db: &State<Database>,
  body: Json<RegisterRequest>,
) -> ApiResult<SuccessMessage> {
  return controller::register(db, body).await;
}

#[utoipa::path(
    post,
    operation_id = "Login 2FA",
    path = "/auth/2fa",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = SuccessMessage),
    ),
    request_body = TwoFactorCodeRequest,
)]
#[post("/auth/2fa", format = "application/json", data = "<body>")]
pub(crate) async fn two_factor(
  db: &State<Database>,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResult<LoginResponse> {
  return controller::two_factor(db, body).await;
}

#[utoipa::path(
    post,
    operation_id = "Login 2FA Recovery",
    path = "/auth/2fa/recovery",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged out", body = SuccessMessage),
    ),
    request_body = TwoFactorRecoveryRequest,
)]
#[post("/auth/2fa/recovery", format = "application/json", data = "<body>")]
pub(crate) async fn two_factor_recovery(
  db: &State<Database>,
  body: Json<TwoFactorRecoveryRequest>,
) -> ApiResult<LoginResponse> {
  return controller::two_factor_recovery(db, body).await;
}

#[utoipa::path(
    post,
    operation_id = "Forgot Password",
    path = "/auth/password/forgot",
    tag = "Auth",
    responses(
        (status = 200, description = "The magic was sent", body = SuccessMessage),
    ),
    request_body = ForgotPasswordRequest,
)]
#[post("/auth/password/forgot", format = "application/json", data = "<body>")]
pub(crate) async fn forgot_password(
  db: &State<Database>,
  body: Json<ForgotPasswordRequest>,
) -> ApiResult<SuccessMessage> {
  controller::forgot_password(db, body).await
}

#[utoipa::path(
    post,
    operation_id = "Reset Password",
    path = "/auth/password/reset",
    tag = "Auth",
    responses(
        (status = 200, description = "Your password was reset", body = SuccessMessage),
    ),
    request_body = ResetPasswordRequest,
)]
#[post("/auth/password/reset", format = "application/json", data = "<body>")]
pub(crate) async fn reset_password(
  db: &State<Database>,
  body: Json<ResetPasswordRequest>,
) -> ApiResult<SuccessMessage> {
  controller::reset_password(db, body).await
}
