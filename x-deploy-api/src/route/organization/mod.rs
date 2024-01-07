use crate::guard::token::Token;
use crate::route::organization::dto::{
  CreateOrganizationRequest, DeleteOrganizationRequest,
  OrganizationInfoResponse, TransferOrganizationRequest,
  UpdateOrganizationRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) mod api_key;
mod controller;
pub(crate) mod credentials;
pub(crate) mod dto;
pub(crate) mod invitation;
pub(crate) mod member;
pub(crate) mod project;
pub(crate) mod role;

#[utoipa::path(
    get,
    operation_id = "Get All Organizations",
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
) -> ApiResult<Vec<OrganizationInfoResponse>> {
  controller::all(db, token).await
}

#[utoipa::path(
    post,
    operation_id = "Create Organization",
    path = "/organization",
    tag = "Organization",
    responses(
        (status = 200, description = "Create a new organization", body = SuccessMessage),
    ),
    request_body = CreateOrganizationRequest,
)]
#[post("/organization", format = "application/json", data = "<body>")]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, token, body).await
}

#[utoipa::path(
    get,
    operation_id = "Get Organization by Id",
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
) -> ApiResult<OrganizationInfoResponse> {
  controller::get_by_id(db, token, id).await
}

#[utoipa::path(
    patch,
    operation_id = "Update Organization",
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Update an organization by id", body = SuccessMessage),
    ),
    request_body = UpdateOrganizationRequest,
)]
#[patch("/organization/<id>", format = "application/json", data = "<body>")]
pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<UpdateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, token, id, body).await
}

#[utoipa::path(
    delete,
    operation_id = "Delete Organization",
    path = "/organization/<id>",
    tag = "Organization",
    responses(
        (status = 200, description = "Delete organization by id", body = SuccessMessage),
    ),
    request_body = DeleteOrganizationRequest,
)]
#[delete("/organization/<id>", format = "application/json", data = "<body>")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<DeleteOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, token, id, body).await
}

#[utoipa::path(
    post,
    operation_id = "Transfer Organization",
    path = "/organization/<id>/transfer",
    tag = "Organization",
    responses(
        (status = 200, description = "Transfer organization by id", body = SuccessMessage),
    ),
    request_body = TransferOrganizationRequest,
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
  body: Json<TransferOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::transfer(db, token, id, body).await
}
