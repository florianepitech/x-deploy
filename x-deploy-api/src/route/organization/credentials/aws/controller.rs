use crate::guard::bearer_token::BearerToken;
use crate::permission::general::{
  verify_general_permission, GeneralPermission,
};
use crate::route::organization::credentials::aws::dto::{
  AwsCredentialsInfoResponse, NewAwsCredentialsRequest,
  UpdateAwsCredentialsRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::object_id::ToObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use x_deploy_common::db::organization_credential_aws::OrganizationCredentialAws;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::CommonCollection;

pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  body: Json<NewAwsCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let org_user = org_member_coll.get_user_in_org(&org_id, &user_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermission::Credentials,
        &StandardPermission::ReadWrite,
      )?;
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
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn get(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<AwsCredentialsInfoResponse> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let cred_id = cred_id.to_object_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let org_user = org_member_coll.get_user_in_org(&org_id, &user_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermission::Credentials,
        &StandardPermission::Read,
      )?;
      // Get credential from database
      let org_cred_aws = CommonCollection::<OrganizationCredentialAws>::new(db);
      let cred_db =
        org_cred_aws.get_by_id_and_org_id(&cred_id, &org_id).await?;
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
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn get_all(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<AwsCredentialsInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let org_user = org_member_coll.get_user_in_org(&org_id, &user_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermission::Credentials,
        &StandardPermission::Read,
      )?;
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
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  cred_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let cred_id = cred_id.to_object_id()?;

  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let org_user = org_member_coll.get_user_in_org(&org_id, &user_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermission::Credentials,
        &StandardPermission::ReadWrite,
      )?;
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
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  cred_id: &str,
  body: Json<UpdateAwsCredentialsRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let cred_id = cred_id.to_object_id()?;

  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let org_user = org_member_coll.get_user_in_org(&org_id, &user_id).await?;
  return match org_user {
    Some(org_user) => {
      // Verify permission
      verify_general_permission(
        org_user.role,
        &GeneralPermission::Credentials,
        &StandardPermission::ReadWrite,
      )?;
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
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}
