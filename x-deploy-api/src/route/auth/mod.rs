use crate::route::auth::dto::{
  LoginBody, LoginResponse, RegisterBody, TwoFactorCode, TwoFactorRecoveryBody,
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
    request_body = LoginBody,
)]
#[post("/auth/login", format = "application/json", data = "<body>")]
pub(crate) async fn login(
  db: &State<Database>,
  body: Json<LoginBody>,
) -> ApiResponse<LoginResponse> {
  return controller::login(db, body).await;
}

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now registered", body = Message)
    ),
    request_body = RegisterBody,
)]
#[post("/auth/register", format = "application/json", data = "<body>")]
pub(crate) async fn register(
  db: &State<Database>,
  body: Json<RegisterBody>,
) -> ApiResponse<SuccessMessage> {
  return controller::register(db, body).await;
}

#[utoipa::path(
    post,
    path = "/auth/2fa",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = Message),
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
        (status = 200, description = "You're now logged out", body = Message),
    ),
    request_body = TwoFactorRecoveryBody,
)]
#[post("/auth/2fa/recovery", format = "application/json", data = "<body>")]
pub(crate) async fn two_factor_recovery(
  db: &State<Database>,
  body: Json<TwoFactorRecoveryBody>,
) -> ApiResponse<LoginResponse> {
  return controller::two_factor_recovery(db, body).await;
}
