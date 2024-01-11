use crate::guard::token::Token;
use crate::permission::general::{
  verify_general_permission, GeneralPermissionType,
};
use crate::route::organization::credentials::docker_hub::dto::{
  DockerHubInfoResponse, NewDockerHubRequest, UpdateDockerHubCredentialsRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::object_id::ToObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use x_deploy_common::db::organization_credential_docker_hub::OrganizationCredentialDockerHub;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::StandardPermission;

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  body: Json<NewDockerHubRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let org_user =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermissionType::Credentials,
        &StandardPermission::ReadWrite,
      )?;
      // Insert credential in database
      let to_insert = OrganizationCredentialDockerHub::new(
        org_id,
        body.name.clone(),
        body.description.clone(),
        body.access_token.clone(),
      );
      to_insert.insert(db).await?;
      // Return success
      return custom_message(
        Status::Created,
        "Successfully created new Docker Hub credential",
      );
    }
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<DockerHubInfoResponse> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let cred_id = cred_id.to_object_id()?;
  let org_user =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermissionType::Credentials,
        &StandardPermission::Read,
      )?;
      // Get credential from database
      let credential_db =
        OrganizationCredentialDockerHub::find_by_id(db, &org_id, &cred_id)
          .await?;
      return match credential_db {
        Some(credential_db) => {
          // Convert to response
          let credential_info = DockerHubInfoResponse {
            id: credential_db.id.to_string(),
            name: credential_db.name,
            description: credential_db.description,
            access_token: credential_db.access_token,
          };
          custom_response(Status::Ok, credential_info)
        }
        None => {
          custom_error(Status::NotFound, "Docker Hub credential not found")
        }
      };
    }
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResult<Vec<DockerHubInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let org_user =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermissionType::Credentials,
        &StandardPermission::Read,
      )?;
      // Get credentials from database
      let credentials_db =
        OrganizationCredentialDockerHub::find_all_for_org(db, &org_id).await?;
      // Convert to response
      let mut result: Vec<DockerHubInfoResponse> = Vec::new();
      for credential in credentials_db {
        let credential_info = DockerHubInfoResponse {
          id: credential.id.to_string(),
          name: credential.name,
          description: credential.description,
          access_token: credential.access_token,
        };
        result.push(credential_info);
      }
      return custom_response(Status::Ok, result);
    }
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let cred_id = cred_id.to_object_id()?;

  let org_user =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermissionType::Credentials,
        &StandardPermission::ReadWrite,
      )?;
      let deleted =
        OrganizationCredentialDockerHub::delete_by_id(db, &org_id, &cred_id)
          .await?;
      return if deleted.deleted_count >= 0 {
        custom_message(Status::Ok, "Successfully deleted Docker Hub credential")
      } else {
        custom_error(Status::NotFound, "Docker Hub credential not found")
      };
    }
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateDockerHubCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let cred_id = cred_id.to_object_id()?;

  let org_user =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermissionType::Credentials,
        &StandardPermission::ReadWrite,
      )?;
      // Get credential from database
      let mut credential_db =
        OrganizationCredentialDockerHub::find_by_id(db, &org_id, &cred_id)
          .await?;
      return match credential_db {
        Some(mut credential_db) => {
          // Update credential
          credential_db.name = body.name.clone();
          credential_db.description = body.description.clone();
          let result = credential_db.update(db).await?;
          if result.matched_count == 0 {
            return custom_error(
              Status::NotFound,
              "Docker Hub credential not found",
            );
          }
          // Return success
          custom_message(
            Status::Ok,
            "Successfully updated Docker Hub credential",
          )
        }
        None => {
          custom_error(Status::NotFound, "Docker Hub credential not found")
        }
      };
    }
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}
