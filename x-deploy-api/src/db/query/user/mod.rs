use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::error::ApiError;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::State;

pub(crate) mod two_factor;

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
