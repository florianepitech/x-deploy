use crate::db::organization_apikey::{
  OrganizationApiKey, ORGANIZATION_APIKEY_COLLECTION_NAME,
};
use crate::guard::token::Token;
use crate::route::organization::api_key::dto::CreateApiKeyRequest;
use crate::route::{custom_message, ApiResponse, SuccessMessage};
use crate::CONFIG;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<CreateApiKeyRequest>,
) -> ApiResponse<SuccessMessage> {
  let objectId = ObjectId::from_str(&id);
  if objectId.is_err() {
    return custom_message(Status::BadRequest, "Invalid organization id");
  }
  let organization_id = objectId.unwrap();
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: String,
  key_id: String,
) -> ApiResponse<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  key_id: String,
) -> ApiResponse<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

async fn verify_max_api_key(
  db: &State<Database>,
  organization_id: ObjectId,
) -> Option<String> {
  let collection: Collection<OrganizationApiKey> =
    db.collection(ORGANIZATION_APIKEY_COLLECTION_NAME);
  // Count the number of api key for the organization
  let count = collection
    .count_documents(doc! {"organizationId": organization_id}, None)
    .await;
  return match count {
    Ok(count) => {
      let max_key_by_org = CONFIG.max_apikey_by_organization;
      if count >= max_key_by_org {
        let error_message = format!(
          "You have reached the maximum of {} api keys for your organization",
          max_key_by_org
        );
        return Some(error_message);
      }
      None
    }
    Err(_) => {
      Some(
        "An error occurred while counting the number of api key for your organization".to_string(),
      )
    }
  };
}
