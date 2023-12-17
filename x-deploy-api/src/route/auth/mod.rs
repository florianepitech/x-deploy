use crate::cipher::password::verify_password;
use crate::cipher::token::{gen_new_token, Token};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::route::auth::dto::{AccountInfo, LoginBody, LoginResponse, RegisterBody};
use crate::route::{CustomResult, Message, MessageResult};
use crate::DOTENV_CONFIG;
use bson::doc;
use bson::oid::ObjectId;
use k8s_openapi::chrono;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

pub mod dto;
mod controller;

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
) -> CustomResult<LoginResponse> {
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
) -> MessageResult {
    return controller::register(db, body).await;
}

#[get("/auth/info")]
pub(crate) async fn info(
    db: &State<Database>,
    token: Token,
) -> CustomResult<AccountInfo> {
    return controller::info(db, token).await;
}
