use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::api_key::dto::{
  ApiKeyInfoResponse, ApiKeyRoleInfoResponse, CreateApiKeyRequest,
  CreateApiKeyResponse, UpdateApiKeyRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::api_key::new_key_value;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_apikey::OrganizationApiKey;
use x_deploy_common::db::organization_role::{
  OrganizationRole, StandardPermission,
};
use x_deploy_common::db::CommonCollection;

pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
  body: Json<CreateApiKeyRequest>,
) -> ApiResult<CreateApiKeyResponse> {
  let body = body.into_inner();
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(&org_id)?;

  GeneralPermission::ApiKeys
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  // Verify role id is valid
  let org_role: Option<OrganizationRole> = match body.role_id {
    Some(role_id) => {
      let role_id = ObjectId::from_str(&role_id)?;
      let orc = CommonCollection::<OrganizationRole>::new(db);
      let role = orc.get_with_id_of_org(&org_id, &role_id).await?;
      if let None = role {
        return custom_error(Status::NotFound, "Role not found");
      }
      role
    }
    None => None,
  };

  // Insert api key in database
  let role_id = match org_role {
    Some(role) => Some(role.id),
    None => None,
  };
  // Verify expired date
  let chrono_expired: Option<bson::DateTime> = match body.expires_at {
    Some(expires_at) => {
      let expires_at: DateTime<Utc> =
        DateTime::from(DateTime::parse_from_rfc3339(&expires_at)?);
      if expires_at < Utc::now() {
        return custom_error(Status::BadRequest, "Expired date is in the past");
      }
      Some(bson::DateTime::from_chrono(expires_at))
    }
    None => None,
  };
  let ak_value = new_key_value();
  let new_api_key = OrganizationApiKey::new(
    body.name.clone(),
    body.description.clone(),
    ak_value.clone(),
    org_id,
    role_id,
    chrono_expired,
  );
  let akc = CommonCollection::<OrganizationApiKey>::new(db);
  akc.insert_one(&new_api_key).await?;

  // Return success
  let response = CreateApiKeyResponse {
    id: new_api_key.id.to_string(),
    key: ak_value,
  };
  custom_response(Status::Created, response)
}

pub(crate) async fn get(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
) -> ApiResult<Vec<ApiKeyInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(&org_id)?;

  GeneralPermission::ApiKeys
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  let akc = CommonCollection::<OrganizationApiKey>::new(db);
  let api_keys = akc.get_all_of_org(&org_id).await?;

  let mut result: Vec<ApiKeyInfoResponse> = Vec::new();
  for api_key in api_keys {
    let role: Option<ApiKeyRoleInfoResponse> = match api_key.role {
      Some(role) => Some(ApiKeyRoleInfoResponse {
        id: role.id.to_string(),
        name: role.name,
        description: role.description,
      }),
      None => None,
    };
    let expires_at: Option<String> = match api_key.expires_at {
      Some(expires_at) => Some(
        expires_at
          .to_chrono()
          .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
      ),
      None => None,
    };
    let api_key_info = ApiKeyInfoResponse {
      id: api_key.id.to_string(),
      name: api_key.name,
      description: api_key.description,
      created_at: api_key
        .id
        .timestamp()
        .to_chrono()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
      expires_at,
      role,
      organization_id: org_id.to_string(),
    };
    result.push(api_key_info);
  }
  custom_response(Status::Ok, result)
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
  key_id: String,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(&org_id)?;

  GeneralPermission::ApiKeys
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  return custom_message(Status::NotImplemented, "Not implemented");
}

pub async fn update(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
  key_id: String,
  body: Json<UpdateApiKeyRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(&org_id)?;
  let key_id = ObjectId::from_str(&key_id)?;

  GeneralPermission::ApiKeys
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  org_id: String,
  key_id: String,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = ObjectId::from_str(&org_id)?;
  let key_id = ObjectId::from_str(&key_id)?;

  GeneralPermission::ApiKeys
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  return custom_message(Status::NotImplemented, "Not implemented");
}
