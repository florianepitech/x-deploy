use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::route::organization::project::dto::{
  CreateProjectRequest, ProjectInfoResponse, UpdateProjectInfoRequest,
};
use crate::route::{ApiResult, SuccessMessage};
use bson::doc;
use mongodb::Database;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::{Data, State};

mod controller;
pub(crate) mod dto;

#[utoipa::path(
  get,
  operation_id = "Get All Projects",
  path = "/organization/<org_id>/project",
  tag = "Organization Projects",
  responses(
    (status = 200, description = "Get all projects", body = Vec<ProjectInfoResponse>),
  ),
)]
#[get("/organization/<org_id>/project", format = "application/json")]
pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<ProjectInfoResponse>> {
  controller::get_all(db, auth, org_id).await
}

#[utoipa::path(
  get,
  operation_id = "Get Project by Id",
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
  token: BearerToken,
  org_id: &str,
  project_id: &str,
) -> ApiResult<ProjectInfoResponse> {
  controller::get_by_id(db, token, org_id, project_id).await
}

#[utoipa::path(
  post,
  operation_id = "Create Project",
  path = "/organization/<org_id>/project",
  tag = "Organization Projects",
  request_body = CreateProjectRequest
)]
#[post(
  "/organization/<org_id>/project",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  body: Json<CreateProjectRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, auth, org_id, body).await
}

#[utoipa::path(
  patch,
  operation_id = "Update Project",
  path = "/organization/<org_id>/project/<project_id>",
  tag = "Organization Projects",
  request_body = UpdateProjectInfoRequest
)]
#[patch(
  "/organization/<org_id>/project/<project_id>",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  project_id: &str,
  body: Json<UpdateProjectInfoRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, token, org_id, project_id, body).await
}

#[utoipa::path(
  post,
  operation_id = "Update Project Logo",
  path = "/organization/<org_id>/project/<project_id>/logo",
  tag = "Organization Projects"
)]
#[post(
  "/organization/<org_id>/project/<project_id>/logo",
  format = "image/*",
  data = "<body>"
)]
pub async fn update_logo(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  project_id: &str,
  content_type: &ContentType,
  body: Data<'_>,
) -> ApiResult<SuccessMessage> {
  controller::update_logo(db, token, org_id, project_id, content_type, body)
    .await
}

#[utoipa::path(
  delete,
  operation_id = "Delete Project",
  path = "/organization/<org_id>/project/<project_id>",
  tag = "Organization Projects"
)]
#[delete(
  "/organization/<org_id>/project/<project_id>",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  project_id: &str,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, token, org_id, project_id).await
}
