use mongodb::Database;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::route::Message;

mod dto;
pub(crate) mod api_key;

#[openapi(tag = "Account")]
#[get("/account", format = "application/json")]
pub(crate) async fn get_info(
    db: &State<Database>,
) -> Result<Json<dto::GetAccountInfoResponse>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[openapi(tag = "Account")]
#[post("/account/verify-email", format = "application/json", data = "<body>")]
pub(crate) async fn verify_email(
    db: &State<Database>,
    body: Json<dto::VerifyEmailBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[openapi(tag = "Account")]
#[post("/account/change-password", format = "application/json", data = "<body>")]
pub(crate) async fn change_password(
    db: &State<Database>,
    body: Json<dto::ChangePasswordBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[openapi(tag = "Account")]
#[post("/account/change-phone", format = "application/json", data = "<body>")]
pub(crate) async fn change_phone(
    db: &State<Database>,
    body: Json<dto::ChangePhoneBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}