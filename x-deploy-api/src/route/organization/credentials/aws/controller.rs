use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::credentials::aws::dto::{
  AwsCredentialsInfoResponse, NewAwsCredentialsRequest,
  UpdateAwsCredentialsRequest,
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
use x_deploy_common::db::organization_credential_aws::OrganizationCredentialAws;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::CommonCollection;

pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  body: Json<NewAwsCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  // Insert credential in database
  let org_cred_aws = CommonCollection::<OrganizationCredentialAws>::new(db);
  let to_insert = OrganizationCredentialAws::new(
    org_id,
    body.name.clone(),
    body.description.clone(),
    body.access_key.clone(),
    body.secret_key.clone(),
  );
  org_cred_aws.insert_one(&to_insert).await?;
  // Return success
  return custom_message(
    Status::Created,
    "Successfully created new Aws credential",
  );
}

pub(crate) async fn get(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<AwsCredentialsInfoResponse> {
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  // Get credential from database
  let org_cred_aws = CommonCollection::<OrganizationCredentialAws>::new(db);
  let cred_db = org_cred_aws.get_by_id_and_org_id(&cred_id, &org_id).await?;
  return match cred_db {
    Some(credential_db) => {
      // Convert to response
      let created_at = credential_db.id.timestamp().to_chrono().to_string();
      let credential_info = AwsCredentialsInfoResponse {
        id: credential_db.id.to_string(),
        name: credential_db.name,
        description: credential_db.description,
        created_at,
      };
      custom_response(Status::Ok, credential_info)
    }
    None => custom_error(Status::NotFound, "Aws credential not found"),
  };
}

pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<AwsCredentialsInfoResponse>> {
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  // Get credentials from database
  let org_cred_aws = CommonCollection::<OrganizationCredentialAws>::new(db);
  let credentials_db = org_cred_aws.get_all_of_org(&org_id).await?;
  // Convert to response
  let mut result: Vec<AwsCredentialsInfoResponse> = Vec::new();
  for credential in credentials_db {
    let created_at = credential.id.timestamp().to_chrono().to_string();
    let credential_info = AwsCredentialsInfoResponse {
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
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  let org_cred_aws = CommonCollection::<OrganizationCredentialAws>::new(db);
  let deleted = org_cred_aws
    .delete_by_id_and_org_id(&cred_id, &org_id)
    .await?;
  return if deleted.deleted_count >= 0 {
    custom_message(Status::Ok, "Successfully deleted Aws credential")
  } else {
    custom_error(Status::NotFound, "Aws credential not found")
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateAwsCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(org_id)?;
  let cred_id = ObjectId::from_str(cred_id)?;

  GeneralPermission::Credentials
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  // Get credential from database
  let org_cred_aws = CommonCollection::<OrganizationCredentialAws>::new(db);
  let credential_db =
    org_cred_aws.get_by_id_and_org_id(&cred_id, &org_id).await?;
  return match credential_db {
    Some(_) => {
      // Update credential
      org_cred_aws
        .update_info(&cred_id, &org_id, &body.name, &body.description)
        .await?;
      // Return success
      custom_message(Status::Ok, "Successfully updated Aws credential")
    }
    None => custom_error(Status::NotFound, "Aws credential not found"),
  };
}
