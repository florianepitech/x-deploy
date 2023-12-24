use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::query::organization::{
  insert_one_organization, verify_number_of_created_organization,
};
use crate::guard::token::Token;
use crate::route::organization::dto::CreateOrganizationBody;
use crate::route::organization::dto::TransferOrganizationBody;
use crate::route::{custom_message, ApiResponse, SuccessMessage};
use bson::oid;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  let owner = oid::ObjectId::parse_str(&token.id).unwrap();
  verify_number_of_created_organization(db, &owner).await?;
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  // Get objectId from token
  let new_organization = Organization::new(
    body.name.clone(),
    body.description.clone(),
    body.website.clone(),
    body.contact_email.clone(),
    owner,
  );
  let result = insert_one_organization(&db, &new_organization).await?;
  let inserted_id = result.inserted_id;
  info!("Inserted new organization with id: {}", inserted_id);
  custom_message(Status::Ok, "Organization created successfully")
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  id: String,
) -> ApiResponse<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn transfer(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<TransferOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
