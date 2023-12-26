use bson::doc;
use mongodb::{Collection, Database};
use mongodb::results::UpdateResult;
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::error::ApiError;

pub(crate) async fn update_password(
  db: &Database,
  id: &bson::oid::ObjectId,
  hash: &str,
) -> Result<UpdateResult, ApiError> {
  let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
  let filter = doc! {
    "_id": id
  };
  let update = doc! {
    "$set": {
      "password": hash
    }
  };
  let result = collection.update_one(filter, update, None).await?;
  return Ok(result);
}