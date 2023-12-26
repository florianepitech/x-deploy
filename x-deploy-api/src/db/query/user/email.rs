use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::error::ApiError;
use bson::{doc, Bson};
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};

pub(crate) async fn confirm_email(
  db: &Database,
  id: &bson::oid::ObjectId,
) -> Result<UpdateResult, ApiError> {
  let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
  let filter = doc! {
    "_id": id
  };
  let update = doc! {
    "$set": {
      "email.verified": true,
      "email.code": Bson::Null
    }
  };
  let result = collection.update_one(filter, update, None).await?;
  return Ok(result);
}
