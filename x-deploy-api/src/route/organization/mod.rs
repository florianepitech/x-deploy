use mongodb::Database;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::route::auth::dto::{LoginBody, LoginResponse};
use crate::route::Message;
use crate::route::organization::dto::CreateOrganizationBody;

pub(crate) mod dto;

#[openapi(tag = "Organization")]
#[post("/organization", format = "application/json", data = "<body>")]
pub(crate) async fn login(
    db: &State<Database>,
    body: Json<CreateOrganizationBody>,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    
}