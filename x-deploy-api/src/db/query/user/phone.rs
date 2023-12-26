use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::error::ApiError;
use bson::doc;
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};

pub(crate) async fn change_phone(
  db: Database,
  id: &bson::oid::ObjectId,
  phone: &str,
) -> Result<UpdateResult, ApiError> {
  let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
  let filter = doc! {
    "_id": id
  };
  let update = doc! {
    "$set": {
      "phone.phone": phone
    }
  };
  let result = collection.update_one(filter, update, None).await?;
  return Ok(result);
}
