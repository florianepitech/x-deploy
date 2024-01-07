use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::CommonResult;
use bson::{doc, Bson};
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};

#[deprecated]
pub async fn query_user_password_update_hash(
  db: &Database,
  id: &bson::oid::ObjectId,
  hash: &str,
) -> CommonResult<UpdateResult> {
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

#[deprecated]
pub async fn query_user_password_update_token(
  db: &Database,
  id: &bson::oid::ObjectId,
  token: Option<&str>,
) -> CommonResult<UpdateResult> {
  let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
  let new_token = match token {
    None => Bson::Null,
    Some(token) => Bson::String(token.to_string()),
  };
  let filter = doc! {
    "_id": id
  };
  let update = doc! {
    "$set": {
      "password.tokenReset": new_token
    }
  };
  let result = collection.update_one(filter, update, None).await?;
  return Ok(result);
}

#[deprecated]
pub async fn query_user_password_from_token(
  db: &Database,
  token_reset: &str,
) -> CommonResult<Option<User>> {
  let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
  let filter = doc! {
    "password.tokenReset": token_reset
  };
  return Ok(collection.find_one(filter, None).await?);
}
