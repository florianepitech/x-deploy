use crate::route::auth::dto::{LoginBody, LoginResponse};
use crate::route::organization::dto::{CreateOrganizationBody, GetByIdQuery};
use crate::route::Message;
use mongodb::Database;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

pub(crate) mod dto;

#[openapi(tag = "Organization")]
#[post("/organization", format = "application/json", data = "<body>")]
pub(crate) async fn new(
    db: &State<Database>,
    body: Json<CreateOrganizationBody>,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[openapi(tag = "Organization")]
#[get("/organization?<query..>", format = "application/json")]
pub(crate) async fn get_by_id(
    db: &State<Database>,
    query: GetByIdQuery,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: format!("Not implemented: {}", query.id),
        }),
    ))
}
