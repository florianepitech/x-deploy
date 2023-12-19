use crate::route::auth::dto::{
  LoginBody, LoginResponse, RegisterBody, TwoFactorCode,
};
use crate::route::{CustomResponse, Message};
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
) -> CustomResponse<LoginResponse> {
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
) -> CustomResponse<Message> {
  return controller::register(db, body).await;
}

#[utoipa::path(
    post,
    path = "/auth/two-factor",
    tag = "Auth",
    responses(
        (status = 200, description = "You're now logged in", body = Message),
    ),
    request_body = TwoFactorCode,
)]
#[post("/auth/two-factor", format = "application/json", data = "<body>")]
pub(crate) async fn two_factor(
  db: &State<Database>,
  body: Json<TwoFactorCode>,
) -> CustomResponse<Message> {
  return controller::two_factor(db, body).await;
}
