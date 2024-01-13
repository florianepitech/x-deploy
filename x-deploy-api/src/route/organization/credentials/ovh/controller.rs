use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::credentials::ovh::dto::{
  NewOvhCredentialsRequest, OvhCredentialsInfoResponse,
  UpdateOvhCredentialsRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_credential_ovh::OrganizationCredentialOvh;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::CommonCollection;

pub async fn new(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  body: Json<NewOvhCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Credentials
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  // Insert credential in database
  let org_cred_ovh = CommonCollection::<OrganizationCredentialOvh>::new(db);
  let to_insert = OrganizationCredentialOvh::new(
    org_id,
    body.name.clone(),
    body.description.clone(),
    body.application_key.clone(),
    body.application_secret.clone(),
    body.consumer_key.clone(),
  );
  org_cred_ovh.insert_one(&to_insert).await?;
  // Return success
  custom_message(Status::Created, "Successfully created new Ovh credential")
}

pub async fn get(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<OvhCredentialsInfoResponse> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  // Get credential from database
  let org_cred_ovh = CommonCollection::<OrganizationCredentialOvh>::new(db);
  let cred_db = org_cred_ovh.get_by_id_and_org_id(&cred_id, &org_id).await?;
  return match cred_db {
    Some(credential_db) => {
      // Convert to response
      let created_at = credential_db.id.timestamp().to_chrono().to_string();
      let credential_info = OvhCredentialsInfoResponse {
        id: credential_db.id.to_string(),
        name: credential_db.name,
        description: credential_db.description,
        created_at,
      };
      custom_response(Status::Ok, credential_info)
    }
    None => custom_error(Status::NotFound, "Ovh credential not found"),
  };
}

pub async fn get_all(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<OvhCredentialsInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Credentials
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  // Get credentials from database
  let org_cred_ovh = CommonCollection::<OrganizationCredentialOvh>::new(db);
  let credentials_db = org_cred_ovh.get_all_of_org(&org_id).await?;
  // Convert to response
  let mut result: Vec<OvhCredentialsInfoResponse> = Vec::new();
  for credential in credentials_db {
    let created_at = credential.id.timestamp().to_chrono().to_string();
    let credential_info = OvhCredentialsInfoResponse {
      id: credential.id.to_string(),
      name: credential.name,
      description: credential.description,
      created_at,
    };
    result.push(credential_info);
  }
  return custom_response(Status::Ok, result);
}

pub async fn delete(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  let org_cred_ovh = CommonCollection::<OrganizationCredentialOvh>::new(db);
  let deleted = org_cred_ovh
    .delete_by_id_and_org_id(&cred_id, &org_id)
    .await?;
  return if deleted.deleted_count >= 0 {
    custom_message(Status::Ok, "Successfully deleted Ovh credential")
  } else {
    custom_error(Status::NotFound, "Ovh credential not found")
  };
}

pub async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateOvhCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  // Get credential from database
  let org_cred_ovh = CommonCollection::<OrganizationCredentialOvh>::new(db);
  let cred_db = org_cred_ovh.get_by_id_and_org_id(&cred_id, &org_id).await?;
  return match cred_db {
    Some(_) => {
      // Update credential
      org_cred_ovh
        .update_info(&cred_id, &org_id, &body.name, &body.description)
        .await?;
      // Return success
      custom_message(Status::Ok, "Successfully updated Ovh credential")
    }
    None => custom_error(Status::NotFound, "Ovh credential not found"),
  };
}
