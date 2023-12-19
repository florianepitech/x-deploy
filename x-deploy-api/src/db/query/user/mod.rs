use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::route::Message;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) mod two_factor;

pub(crate) async fn get_user_from_db(
  db: &State<Database>,
  user_id: &ObjectId,
) -> Result<User, Custom<Json<Message>>> {
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
    .await;
  if (user.is_err()) {
    return Err(Custom(
      Status::InternalServerError,
      Json(Message::new(
        "Error while getting user from database".to_string(),
      )),
    ));
  }
  let user = user.unwrap();

  return match user {
    Some(user) => Ok(user),
    None => Err(Custom(
      Status::NotFound,
      Json(Message::new("You're account doesn't exist !".to_string())),
    )),
  };
}
