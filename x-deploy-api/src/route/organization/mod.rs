use crate::guard::token::Token;
use crate::route::organization::dto::{
  CreateOrganizationBody, TransferOrganizationBody,
};
use crate::route::{ApiResponse, SuccessMessage};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) mod api_key;
mod controller;
pub(crate) mod credentials;
pub(crate) mod dto;
pub(crate) mod member;
pub(crate) mod project;

enum CloudProvider {
  Ovh,
  Aws,
  Azure,
  GoogleCloud,
}

#[utoipa::path(
    post,
    path = "/organization",
    tag = "Organization",
    responses(
        (status = 200, description = "Create a new organization", body = Message),
    ),
    request_body = CreateOrganizationBody,
)]
#[post("/organization", format = "application/json", data = "<body>")]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  controller::new(db, token, body).await
}

#[utoipa::path(
    get,
    path = "/organization",
    tag = "Organization",
    responses(
        (status = 200, description = "Get organization by id", body = Message),
    )
)]
#[get("/organization/<id>", format = "application/json")]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  id: String,
) -> ApiResponse<SuccessMessage> {
  controller::get_by_id(db, id).await
}

#[utoipa::path(
    get,
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Get organization by id", body = Message),
    ),
)]
#[patch("/organization/<id>", format = "application/json")]
pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  controller::update(db, token, id).await
}

#[utoipa::path(
    delete,
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Delete organization by id", body = Message),
    ),
)]
#[delete("/organization/<id>", format = "application/json")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  controller::delete(db, token, id).await
}

#[utoipa::path(
    post,
    path = "/organization/<id>/transfer",
    tag = "Organization",
    responses(
        (status = 200, description = "Transfer organization by id", body = Message),
    ),
    request_body = TransferOrganizationBody,
)]
#[post(
  "/organization/<id>/transfer",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn transfer(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<TransferOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  controller::transfer(db, token, id, body).await
}
