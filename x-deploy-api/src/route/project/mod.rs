use crate::route::auth::dto::LoginResponse;
use crate::route::project::dto::CreateProjectBody;
use crate::route::Message;
use mongodb::Database;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

mod dto;

#[openapi(tag = "Project")]
#[post("/project", format = "application/json", data = "<body>")]
pub(crate) async fn new(
    db: &State<Database>,
    body: Json<CreateProjectBody>,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: "Not implemented".to_string(),
        }),
    ))
}

#[openapi(tag = "Project")]
#[get("/project?<query..>", format = "application/json")]
pub(crate) async fn get_by_id(
    db: &State<Database>,
    query: dto::GetByIdQuery,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: format!("Not implemented: {}", query.id),
        }),
    ))
}