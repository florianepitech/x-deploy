use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::error::ApiError;
use crate::DOTENV_CONFIG;
use bson::{doc, oid};
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::State;

pub(crate) async fn insert_one_organization(
  db: &State<Database>,
  organization: &Organization,
) -> Result<InsertOneResult, ApiError> {
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  let result = collection.insert_one(organization, None).await?;
  return Ok(result);
}

pub(crate) async fn verify_number_of_created_organization(
  db: &State<Database>,
  user_id: &oid::ObjectId,
) -> Result<(), ApiError> {
  let max_by_owner = DOTENV_CONFIG.max_organization_by_owner;
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  let count = collection
    .count_documents(
      doc! {
          "owner": user_id
      },
      None,
    )
    .await?;
  if count >= max_by_owner {
    let message =
      format!("You can't create more than {} organization", max_by_owner);
    return Err(ApiError::new(Status::Forbidden, message));
  }
  return Ok(());
}
