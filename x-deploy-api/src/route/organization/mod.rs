use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::route::organization::dto::{
  CreateOrganizationRequest, DeleteOrganizationRequest,
  OrganizationInfoResponse, TransferOrganizationRequest,
  UpdateOrganizationRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::{Data, State};

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
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Get all organizations", body = Vec<OrganizationInfoResponse>),
    ),
)]
#[get("/organization", format = "application/json")]
pub(crate) async fn all(
  db: &State<Database>,
  token: BearerToken,
) -> ApiResult<Vec<OrganizationInfoResponse>> {
  controller::all(db, token).await
}

#[utoipa::path(
    post,
    operation_id = "Create Organization",
    path = "/organization",
    tag = "Organization",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Create a new organization", body = SuccessMessage),
    ),
    request_body = CreateOrganizationRequest,
)]
#[post("/organization", format = "application/json", data = "<body>")]
pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  body: Json<CreateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, token, body).await
}

#[utoipa::path(
    get,
    operation_id = "Get Organization by Id",
    path = "/organization/<id>",
    tag = "Organization",
    security(
      ("bearer" = []),
      ("apiKey" = []),
    ),
    responses(
        (status = 200, description = "Get organization by id", body = OrganizationInfoResponse),
    )
)]
#[get("/organization/<id>", format = "application/json")]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  auth: Auth,
  id: String,
) -> ApiResult<OrganizationInfoResponse> {
  controller::get_by_id(db, auth, id).await
}

#[utoipa::path(
    patch,
    operation_id = "Update Organization",
    path = "/organization/<id>",
    tag = "Organization",
    security(
      ("bearer" = []),
      ("apiKey" = []),
    ),
    responses(
        (status = 200, description = "Update an organization by id", body = SuccessMessage),
    ),
    request_body = UpdateOrganizationRequest,
)]
#[patch("/organization/<id>", format = "application/json", data = "<body>")]
pub(crate) async fn update(
  db: &State<Database>,
  auth: Auth,
  id: String,
  body: Json<UpdateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, auth, id, body).await
}

#[utoipa::path(
    post,
    operation_id = "Upload Organization Logo",
    path = "/organization/<org_id>/logo",
    tag = "Organization",
    security(
      ("bearer" = []),
      ("apiKey" = []),
    ),
    responses(
        (status = 200, description = "Upload organization logo by id", body = SuccessMessage),
    ),
    request_body = Vec<u8>
)]
#[post("/organization/<org_id>/logo", format = "image/*", data = "<body>")]
pub(crate) async fn update_logo(
  db: &State<Database>,
  auth: Auth,
  org_id: String,
  content_type: &ContentType,
  body: Data<'_>,
) -> ApiResult<SuccessMessage> {
  controller::update_logo(db, auth, org_id, content_type, body).await
}

#[utoipa::path(
    delete,
    operation_id = "Delete Organization",
    path = "/organization/<id>",
    tag = "Organization",
    security(
      ("bearer" = []),
    ),
    responses(
        (status = 200, description = "Delete organization by id", body = SuccessMessage),
    ),
    request_body = DeleteOrganizationRequest,
)]
#[delete("/organization/<id>", format = "application/json", data = "<body>")]
pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  id: String,
  body: Json<DeleteOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, token, id, body).await
}

#[deprecated]
#[utoipa::path(
    post,
    operation_id = "Transfer Organization",
    path = "/organization/<id>/transfer",
    tag = "Organization",
    security(
      ("bearer" = []),
    ),
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
  token: BearerToken,
  id: String,
  body: Json<TransferOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::transfer(db, token, id, body).await
}
