use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::project::Project;
use crate::guard::token::Token;
use crate::route::organization::project::dto::{
  CreateProjectBody, ProjectInfoResponse, UpdateProjectInfoBody,
};
use crate::route::{ApiResponse, SuccessMessage};
use bson::{doc, oid};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub(crate) mod dto;

#[utoipa::path(
  get,
  path = "/organization/<org_id>/project",
  tag = "Organization Projects",
  responses(
    (status = 200, description = "Get all projects", body = Vec<ProjectInfoResponse>),
  ),
)]
#[get("/organization/<org_id>/project", format = "application/json")]
pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResponse<Vec<ProjectInfoResponse>> {
  controller::get_all(db, token, org_id).await
}

#[utoipa::path(
  get,
  path = "/organization/<org_id>/project/<project_id>",
  tag = "Organization Projects",
  responses(
    (status = 200, description = "Get project by id", body = ProjectInfoResponse),
  ),
)]
#[get(
  "/organization/<org_id>/project/<project_id>",
  format = "application/json"
)]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
) -> ApiResponse<ProjectInfoResponse> {
  controller::get_by_id(db, token, org_id, project_id).await
}

#[utoipa::path(
  post,
  path = "/organization/<org_id>/project",
  tag = "Organization Projects",
  request_body = CreateProjectBody
)]
#[post(
  "/organization/<org_id>/project",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  body: Json<CreateProjectBody>,
) -> ApiResponse<SuccessMessage> {
  controller::new(db, token, org_id, body).await
}

#[utoipa::path(
  patch,
  path = "/organization/<org_id>/project/<project_id>",
  tag = "Organization Projects",
  request_body = UpdateProjectInfoBody
)]
#[patch(
  "/organization/<org_id>/project/<project_id>",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
  body: Json<UpdateProjectInfoBody>,
) -> ApiResponse<SuccessMessage> {
  controller::update(db, token, org_id, project_id, body).await
}

#[utoipa::path(
  delete,
  path = "/organization/<org_id>/project/<project_id>",
  tag = "Organization Projects"
)]
#[delete(
  "/organization/<org_id>/project/<project_id>",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
) -> ApiResponse<SuccessMessage> {
  controller::delete(db, token, org_id, project_id).await
}
