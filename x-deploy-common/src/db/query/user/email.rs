use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::CommonResult;
use bson::{doc, Bson};
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};

#[deprecated]
pub async fn confirm_email(
  db: &Database,
  id: &bson::oid::ObjectId,
) -> CommonResult<UpdateResult> {
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
