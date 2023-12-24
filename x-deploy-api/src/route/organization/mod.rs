use crate::guard::token::Token;
use crate::route::organization::dto::{CreateOrganizationBody, DeleteOrganizationBody, OrganizationInfoResponse, TransferOrganizationBody, UpdateOrganizationBody};
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

#[utoipa::path(
    get,
    path = "/organization",
    tag = "Organization",
    responses(
        (status = 200, description = "Get all organizations", body = Vec<OrganizationInfoResponse>),
    ),
)]
#[get("/organization", format = "application/json")]
pub(crate) async fn all(
  db: &State<Database>,
  token: Token,
) -> ApiResponse<Vec<OrganizationInfoResponse>> {
  controller::all(db, token).await
}

#[utoipa::path(
    post,
    path = "/organization",
    tag = "Organization",
    responses(
        (status = 200, description = "Create a new organization", body = SuccessMessage),
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
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Get organization by id", body = OrganizationInfoResponse),
    )
)]
#[get("/organization/<id>", format = "application/json")]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<OrganizationInfoResponse> {
  controller::get_by_id(db, token, id).await
}

#[utoipa::path(
    patch,
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Update an organization by id", body = SuccessMessage),
    ),
    request_body = UpdateOrganizationBody,
)]
#[patch("/organization/<id>", format = "application/json", data = "<body>")]
pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<UpdateOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  controller::update(db, token, id, body).await
}

#[utoipa::path(
    delete,
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Delete organization by id", body = SuccessMessage),
    ),
    request_body = DeleteOrganizationBody,
)]
#[delete("/organization/<id>", format = "application/json", data = "<body>")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<DeleteOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  controller::delete(db, token, id, body).await
}

#[utoipa::path(
    post,
    path = "/organization/<id>/transfer",
    tag = "Organization",
    responses(
        (status = 200, description = "Transfer organization by id", body = SuccessMessage),
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
