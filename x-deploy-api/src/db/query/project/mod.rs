use crate::db::organization::Organization;
use crate::db::project::{Project, PROJECT_COLLECTION_NAME};
use crate::error::ApiError;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;
use mongodb::Database;
use rocket::futures::TryStreamExt;

pub async fn query_project_new(
  db: &Database,
  project: &Project,
) -> Result<InsertOneResult, ApiError> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let result = collection.insert_one(project, None).await?;
  Ok(result)
}

pub async fn query_project_get_with_org(
  db: &Database,
  org_id: &ObjectId,
) -> Result<Vec<Project>, ApiError> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id
  };
  let mut cursor = collection.find(filter, None).await?;
  let mut result: Vec<Project> = Vec::new();
  while let Some(doc) = cursor.try_next().await? {
    result.push(doc);
  }
  return Ok(result);
}

pub async fn query_project_get_with_org_and_id(
  db: &Database,
  org_id: &ObjectId,
  project_id: &ObjectId,
) -> Result<Option<Project>, ApiError> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id,
    "_id": project_id
  };
  let result = collection.find_one(filter, None).await;
  Ok(result?)
}

pub async fn query_project_update(
  db: &Database,
  org_id: &ObjectId,
  project_id: &ObjectId,
  name: &str,
  description: &str,
) -> Result<UpdateResult, ApiError> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id,
    "_id": project_id
  };
  let update = doc! {
    "$set": {
      "name": name,
      "description": description
    }
  };
  let result = collection.update_one(filter, update, None).await?;
  Ok(result)
}

pub async fn query_project_delete(
  db: &Database,
  org_id: &ObjectId,
  project_id: &ObjectId,
) -> Result<DeleteResult, ApiError> {
  let collection: Collection<Project> = db.collection(PROJECT_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": org_id,
    "_id": project_id
  };
  let result = collection.delete_one(filter, None).await?;
  Ok(result)
}
