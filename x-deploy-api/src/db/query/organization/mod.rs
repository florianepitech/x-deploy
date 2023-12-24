use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::error::ApiError;
use crate::DOTENV_CONFIG;
use bson::{doc, oid};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Collection, Database};
use rocket::futures::{StreamExt, TryStreamExt};
use rocket::http::Status;
use rocket::State;

pub(crate) async fn get_all_orgs_of_user(
  db: &State<Database>,
  user_id: &oid::ObjectId,
) -> Result<Vec<Organization>, ApiError> {
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  let mut cursor = collection
    .find(
      doc! {
          "owner": user_id
      },
      None,
    )
    .await?;
  let mut organizations: Vec<Organization> = Vec::new();
  while let Some(doc) = cursor.try_next().await? {
    organizations.push(doc);
  }
  return Ok(organizations);
}

pub(crate) async fn get_org_by_id_with_owner(
  db: &State<Database>,
  owner_id: &oid::ObjectId,
  org_id: &oid::ObjectId,
) -> Result<Organization, ApiError> {
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  let result = collection
    .find_one(
      doc! {
          "_id": org_id,
          "owner": owner_id
      },
      None,
    )
    .await?;
  return match result {
    Some(org) => Ok(org),
    None => {
      let message = format!("Organization with id {} not found", org_id);
      Err(ApiError::new(Status::NotFound, message))
    }
  };
}

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

pub(crate) async fn update_organization_info(
  db: &Database,
  id: &oid::ObjectId,
  name: String,
  description: String,
  website: String,
  contact_email: String,
) -> Result<UpdateResult, ApiError> {
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  let result = collection
    .update_one(
      doc! {
          "_id": id
      },
      doc! {
          "$set": {
              "name": name,
              "description": description,
              "website": website,
              "contactEmail": contact_email
          }
      },
      None,
    )
    .await?;
  return Ok(result);
}

pub(crate) async fn delete_organization_by_id(
  db: &Database,
  id: &oid::ObjectId,
) -> Result<DeleteResult, ApiError> {
  let collection: Collection<Organization> =
    db.collection(ORGANIZATION_COLLECTION_NAME);
  let result = collection
    .delete_one(
      doc! {
          "_id": id
      },
      None,
    )
    .await?;
  return Ok(result);
}
