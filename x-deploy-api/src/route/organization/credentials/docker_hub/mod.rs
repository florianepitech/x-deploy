use crate::guard::token::Token;
use crate::route::organization::credentials::docker_hub::dto::{
  DockerHubInfoResponse, NewDockerHubRequest, UpdateDockerHubCredentialsRequest,
};
use crate::route::{custom_error, custom_message, ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub(crate) mod dto;

#[utoipa::path(
  post,
  operation_id = "Create Docker Hub Credentials",
  path = "/organization/<org_id>/credentials/docker-hub",
  tag = "Organization Credentials Docker Hub",
  responses(
    (status = 201, description = "Successfully created new Docker Hub credential", body = SuccessMessage),
  ),
)]
#[post(
  "/organization/<org_id>/credentials/docker-hub",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  body: Json<NewDockerHubRequest>,
) -> ApiResult<SuccessMessage> {
  controller::new(db, token, org_id, body).await
}

#[utoipa::path(
  get,
  operation_id = "Get Docker Hub Credentials",
  path = "/organization/<org_id>/credentials/docker-hub/<cred_id>",
  tag = "Organization Credentials Docker Hub",
  responses(
    (status = 200, description = "Get Docker Hub credential", body = DockerHubInfoResponse),
  ),
)]
#[get(
  "/organization/<org_id>/credentials/docker-hub/<cred_id>",
  format = "application/json"
)]
pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<DockerHubInfoResponse> {
  controller::get(db, token, org_id, cred_id).await
}

#[utoipa::path(
  get,
  operation_id = "Get All Docker Hub Credentials",
  path = "/organization/<org_id>/credentials/docker-hub",
  tag = "Organization Credentials Docker Hub",
  responses(
    (status = 200, description = "Get all Docker Hub credentials", body = Vec<DockerHubInfoResponse>),
  ),
)]
#[get(
  "/organization/<org_id>/credentials/docker-hub",
  format = "application/json"
)]
pub async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResult<Vec<DockerHubInfoResponse>> {
  controller::get_all(db, token, org_id).await
}

#[utoipa::path(
  patch,
  operation_id = "Update Docker Hub Credentials",
  path = "/organization/<org_id>/credentials/docker-hub/<cred_id>",
  tag = "Organization Credentials Docker Hub",
  responses(
    (status = 200, description = "Successfully updated Docker Hub credential", body = SuccessMessage),
  ),
  request_body = UpdateDockerHubCredentialsRequest,
)]
#[patch(
  "/organization/<org_id>/credentials/docker-hub/<cred_id>",
  format = "application/json",
  data = "<body>"
)]
pub async fn update(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateDockerHubCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  controller::update(db, token, org_id, cred_id, body).await
}

#[utoipa::path(
  delete,
  operation_id = "Delete Docker Hub Credentials",
  path = "/organization/<org_id>/credentials/docker-hub/<cred_id>",
  tag = "Organization Credentials Docker Hub",
  responses(
    (status = 200, description = "Successfully deleted Docker Hub credential", body = SuccessMessage),
  ),
)]
#[delete(
  "/organization/<org_id>/credentials/docker-hub/<cred_id>",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<SuccessMessage> {
  controller::delete(db, token, org_id, cred_id).await
}
