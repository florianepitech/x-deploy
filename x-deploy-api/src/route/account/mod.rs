use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::guard::token::Token;
use crate::route::account::dto::{
  ChangePasswordRequest, GetAccountInfoResponse, TwoFactorCodeRequest,
  TwoFactorInfoRequest, TwoFactorInfoResponse, TwoFactorSetupRequest,
  TwoFactorSetupResponse,
};
use crate::route::{
  custom_message, custom_response, ApiResponse, SuccessMessage,
};
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub(crate) mod dto;

#[utoipa::path(
    get,
    path = "/account",
    tag = "Account",
    security(
      ("token_jwt" = [])
    ),
    responses(
        (status = 200, description = "Get account info", body = GetAccountInfoResponse),
    ),
)]
#[get("/account", format = "application/json")]
pub(crate) async fn get_info(
  token: Token,
  db: &State<Database>,
) -> ApiResponse<GetAccountInfoResponse> {
  return controller::get_info(token, db).await;
}

#[utoipa::path(
    post,
    path = "/account/verify-email",
    tag = "Account",
    responses(
        (status = 200, description = "Verify email", body = SuccessMessage),
    ),
    request_body = VerifyEmailRequest,
)]
#[post("/account/verify-email", format = "application/json", data = "<body>")]
pub(crate) async fn verify_email(
  db: &State<Database>,
  token: Token,
  body: Json<dto::VerifyEmailRequest>,
) -> ApiResponse<SuccessMessage> {
  return controller::verify_email(db, token, body).await;
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/change-password",
    tag = "Account",
    responses(
        (status = 200, description = "Change password", body = SuccessMessage),
    ),
    request_body = ChangePasswordRequest,
)]
#[post(
  "/account/change-password",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn change_password(
  db: &State<Database>,
  token: Token,
  body: Json<ChangePasswordRequest>,
) -> ApiResponse<SuccessMessage> {
  return controller::change_password(db, token, body).await;
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/change-phone",
    tag = "Account",
    responses(
        (status = 200, description = "Change phone", body = SuccessMessage),
    ),
    request_body = ChangePhoneRequest,
)]
#[post("/account/change-phone", format = "application/json", data = "<body>")]
pub(crate) async fn change_phone(
  db: &State<Database>,
  body: Json<dto::ChangePhoneRequest>,
) -> ApiResponse<SuccessMessage> {
  return controller::change_phone(db, body).await;
}

// 2FA

#[deprecated]
#[utoipa::path(
  post,
  path = "/account/2fa",
  tag = "Account",
  responses(
    (status = 200, description = "Information about your 2FA", body = TwoFactorInfoResponse),
  ),
  request_body = TwoFactorInfoRequest,
)]
#[post("/account/2fa", format = "application/json", data = "<body>")]
pub(crate) async fn info_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorInfoRequest>,
) -> ApiResponse<TwoFactorInfoResponse> {
  return controller::info_2fa(db, token, body).await;
}

#[deprecated]
#[utoipa::path(
  post,
  path = "/account/2fa/setup",
  tag = "Account",
  responses(
    (status = 200, description = "The data about your new 2FA setup", body = TwoFactorSetupResponse),
  ),
  request_body = TwoFactorSetupRequest,
)]
#[post("/account/2fa/setup", format = "application/json", data = "<body>")]
pub(crate) async fn setup_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorSetupRequest>,
) -> ApiResponse<TwoFactorSetupResponse> {
  return controller::setup_2fa(db, token, body).await;
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/2fa/enable",
    tag = "Account",
    responses(
        (status = 200, description = "Create api key", body = SuccessMessage),
    ),
    request_body = TwoFactorCodeRequest,
)]
#[post("/account/2fa/enable", format = "application/json", data = "<body>")]
pub(crate) async fn enable_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResponse<SuccessMessage> {
  return controller::enable_2fa(db, token, body).await;
}

#[deprecated]
#[utoipa::path(
    post,
    path = "/account/2fa/disable",
    tag = "Account",
    responses(
        (status = 200, description = "Create api key", body = SuccessMessage),
    ),
    request_body = TwoFactorCodeRequest,
)]
#[post("/account/2fa/disable", format = "application/json", data = "<body>")]
pub(crate) async fn disable_2fa(
  db: &State<Database>,
  token: Token,
  body: Json<TwoFactorCodeRequest>,
) -> ApiResponse<SuccessMessage> {
  return controller::disable_2fa(db, token, body).await;
}
