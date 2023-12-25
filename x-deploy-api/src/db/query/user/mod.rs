use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::error::ApiError;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::State;

pub(crate) mod two_factor;

pub(crate) async fn add_user_to_db(
  db: &State<Database>,
  user: User,
) -> Result<InsertOneResult, ApiError> {
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  let result = collection.insert_one(user, None).await?;
  return Ok(result);
}

pub(crate) async fn get_user_from_db(
  db: &State<Database>,
  user_id: &ObjectId,
) -> Result<User, ApiError> {
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  let user = collection
    .find_one(
      doc! {
          "_id": user_id
      },
      None,
    )
    .await?;
  return match user {
    Some(user) => Ok(user),
    None => Err(ApiError::new(
      Status::NotFound,
      "Your account does not exist".to_string(),
    )),
  };
}

pub(crate) async fn get_user_from_email(
  db: &State<Database>,
  email: &str,
) -> Result<User, ApiError> {
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  let user = collection
    .find_one(
      doc! {
          "email.email": email
      },
      None,
    )
    .await?;
  return match user {
    Some(user) => Ok(user),
    None => Err(ApiError::new(
      Status::NotFound,
      "Your account does not exist".to_string(),
    )),
  };
}
