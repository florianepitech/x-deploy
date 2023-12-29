use crate::route::auth::dto::{
  ForgotPasswordRequest, LoginRequest, LoginResponse, MagicLinkRequest,
  RegisterRequest, ResetPasswordRequest, TwoFactorCode,
  TwoFactorRecoveryRequest,
};
use crate::route::{ApiResponse, SuccessMessage};
use bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

mod controller;
pub mod dto;

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = LoginResponse),
    ),
    request_body = LoginRequest,
)]
#[post("/auth/login", format = "application/json", data = "<body>")]
pub(crate) async fn login(
  db: &State<Database>,
  body: Json<LoginRequest>,
) -> ApiResponse<LoginResponse> {
  return controller::login(db, body).await;
}

#[utoipa::path(
    post,
    path = "/auth/magic-link",
    tag = "Auth",
    responses(
        (status = 200, description = "The magic was sent", body = SuccessMessage),
    ),
)]
#[post("/auth/magic-link", format = "application/json", data = "<body>")]
pub(crate) async fn magic_link(
  db: &State<Database>,
  body: Json<MagicLinkRequest>,
) -> ApiResponse<SuccessMessage> {
  return controller::magic_link(db, body).await;
}

#[utoipa::path(
    post,
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
) -> ApiResponse<SuccessMessage> {
  return controller::register(db, body).await;
}

#[utoipa::path(
    post,
    path = "/auth/2fa",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = SuccessMessage),
    ),
    request_body = TwoFactorCode,
)]
#[post("/auth/2fa", format = "application/json", data = "<body>")]
pub(crate) async fn two_factor(
  db: &State<Database>,
  body: Json<TwoFactorCode>,
) -> ApiResponse<LoginResponse> {
  return controller::two_factor(db, body).await;
}

#[utoipa::path(
    post,
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
) -> ApiResponse<LoginResponse> {
  return controller::two_factor_recovery(db, body).await;
}

#[utoipa::path(
    post,
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
) -> ApiResponse<SuccessMessage> {
  controller::forgot_password(db, body).await
}

#[utoipa::path(
    post,
    path = "/auth/password/reset",
    tag = "Auth",
    responses(
        (status = 200, description = "Your password was reset", body = SuccessMessage),
    ),
)]
#[post("/auth/password/reset", format = "application/json", data = "<body>")]
pub(crate) async fn reset_password(
  db: &State<Database>,
  body: Json<ResetPasswordRequest>,
) -> ApiResponse<SuccessMessage> {
  controller::reset_password(db, body).await
}
