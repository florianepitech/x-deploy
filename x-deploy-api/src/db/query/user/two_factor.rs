use crate::db::user::{TwoFactor, User, USER_COLLECTION_NAME};
use crate::route::Message;
use bson::oid::ObjectId;
use bson::{doc, Bson};
use k8s_openapi::chrono;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use totp_rs::TOTP;

pub(crate) async fn setup_2fa_in_db(
  db: &State<Database>,
  user: &User,
  description: &String,
  totp: &TOTP,
) -> Result<(), Custom<Json<Message>>> {
  let data: TwoFactor = TwoFactor {
    enabled: false,
    secret: totp.secret.clone(),
    created: chrono::Utc::now(),
    description: description.clone(),
  };
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);

  let query = doc! {
    "_id": user.id.clone()
  };
  let update = doc! {
    "$set": {
      "twoFactor": bson::to_bson(&data).unwrap()
    }
  };
  let update_result = collection.update_one(query, update, None).await;

  return match update_result {
    Ok(update) => {
      if (update.modified_count == 0) {
        return Err(Custom(
          Status::InternalServerError,
          Json(Message::new(
            "Error while updating user 2FA in database".to_string(),
          )),
        ));
      }
      Ok(())
    }
    Err(_) => Err(Custom(
      Status::InternalServerError,
      Json(Message::new(
        "Error while updating user 2FA in database".to_string(),
      )),
    )),
  };
}

pub(crate) async fn update_2fa_state_in_db(
  db: &State<Database>,
  user: &User,
  enable: bool,
) -> Result<(), Custom<Json<Message>>> {
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);

  let query = doc! {
    "_id": user.id.clone()
  };
  let update = doc! {
    "$set": {
      "twoFactor.enabled": enable
    }
  };
  let update_result = collection.update_one(query, update, None).await;

  return match update_result {
    Ok(update) => {
      if (update.modified_count == 0) {
        return Err(Custom(
          Status::InternalServerError,
          Json(Message::new(
            "Error while updating user 2FA in database".to_string(),
          )),
        ));
      }
      Ok(())
    }
    Err(_) => Err(Custom(
      Status::InternalServerError,
      Json(Message::new(
        "Error while updating user 2FA in database".to_string(),
      )),
    )),
  };
}

pub(crate) async fn delete_2fa_in_db(
  db: &State<Database>,
  user_id: &ObjectId,
) -> Result<(), Custom<Json<Message>>> {
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);
  let query = doc! {
    "_id": user_id.clone()
  };
  let update = doc! {
    "$et": {
      "twoFactor": Bson::Null
    }
  };
  let update_result = collection.update_one(query, update, None).await;
  return match update_result {
    Ok(update) => {
      if (update.modified_count == 0) {
        return Err(Custom(
          Status::InternalServerError,
          Json(Message::new(
            "Error while disabling user 2FA in database".to_string(),
          )),
        ));
      }
      Ok(())
    }
    Err(_) => Err(Custom(
      Status::InternalServerError,
      Json(Message::new(
        "Error while disabling user 2FA in database".to_string(),
      )),
    )),
  };
}
