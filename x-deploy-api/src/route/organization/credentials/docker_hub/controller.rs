use crate::guard::auth::Auth;
use crate::permission::general::GeneralPermission;
use crate::route::organization::credentials::docker_hub::dto::{
  DockerHubInfoResponse, NewDockerHubRequest, UpdateDockerHubCredentialsRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_credential_docker_hub::OrganizationCredentialDockerHub;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::CommonCollection;

pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  body: Json<NewDockerHubRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = bson::oid::ObjectId::from_str(org_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  // Insert credential in database
  let docker_hub_coll =
    CommonCollection::<OrganizationCredentialDockerHub>::new(db);
  let to_insert = OrganizationCredentialDockerHub::new(
    org_id,
    body.name.clone(),
    body.description.clone(),
    body.access_token.clone(),
  );
  docker_hub_coll.insert_one(&to_insert).await?;
  // Return success
  return custom_message(
    Status::Created,
    "Successfully created new Docker Hub credential",
  );
}

pub(crate) async fn get(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<DockerHubInfoResponse> {
  let org_id = bson::oid::ObjectId::from_str(org_id)?;
  let cred_id = bson::oid::ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  // Get credential from database
  let docker_hub_coll =
    CommonCollection::<OrganizationCredentialDockerHub>::new(db);
  let credential_db = docker_hub_coll
    .get_by_id_and_org_id(&cred_id, &org_id)
    .await?;
  return match credential_db {
    Some(credential_db) => {
      // Convert to response
      let created_at = credential_db.id.timestamp().to_chrono().to_string();
      let credential_info = DockerHubInfoResponse {
        id: credential_db.id.to_string(),
        name: credential_db.name,
        description: credential_db.description,
        created_at,
      };
      custom_response(Status::Ok, credential_info)
    }
    None => custom_error(Status::NotFound, "Docker Hub credential not found"),
  };
}

pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<DockerHubInfoResponse>> {
  let org_id = bson::oid::ObjectId::from_str(org_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  // Get credentials from database
  let docker_hub_coll =
    CommonCollection::<OrganizationCredentialDockerHub>::new(db);
  let credentials_db = docker_hub_coll.get_all_of_org(&org_id).await?;
  // Convert to response
  let mut result: Vec<DockerHubInfoResponse> = Vec::new();
  for credential in credentials_db {
    let created_at = credential.id.timestamp().to_chrono().to_string();
    let credential_info = DockerHubInfoResponse {
      id: credential.id.to_string(),
      name: credential.name,
      description: credential.description,
      created_at,
    };
    result.push(credential_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn delete(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<SuccessMessage> {
  let org_id = bson::oid::ObjectId::from_str(org_id)?;
  let cred_id = bson::oid::ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  let docker_hub_coll =
    CommonCollection::<OrganizationCredentialDockerHub>::new(db);
  let deleted = docker_hub_coll
    .delete_by_id_and_org_id(&cred_id, &org_id)
    .await?;
  return if deleted.deleted_count >= 0 {
    custom_message(Status::Ok, "Successfully deleted Docker Hub credential")
  } else {
    custom_error(Status::NotFound, "Docker Hub credential not found")
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateDockerHubCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = bson::oid::ObjectId::from_str(org_id)?;
  let cred_id = bson::oid::ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  // Get credential from database
  let docker_hub_coll =
    CommonCollection::<OrganizationCredentialDockerHub>::new(db);
  let credential_db = docker_hub_coll
    .get_by_id_and_org_id(&cred_id, &org_id)
    .await?;
  return match credential_db {
    Some(_) => {
      // Update credential
      let result = docker_hub_coll
        .update_info(&cred_id, &org_id, &body.name, &body.description)
        .await?;
      if result.matched_count == 0 {
        return custom_error(
          Status::NotFound,
          "Docker Hub credential not found",
        );
      }
      // Return success
      custom_message(Status::Ok, "Successfully updated Docker Hub credential")
    }
    None => custom_error(Status::NotFound, "Docker Hub credential not found"),
  };
}
