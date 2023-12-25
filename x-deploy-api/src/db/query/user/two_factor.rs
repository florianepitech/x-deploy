use crate::db::user::{TwoFactor, User, USER_COLLECTION_NAME};
use crate::error::ApiError;
use bson::oid::ObjectId;
use bson::{doc, Bson};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::yansi::Paint;
use rocket::State;
use totp_rs::TOTP;

pub(crate) async fn setup_2fa_in_db(
  db: &State<Database>,
  user: &User,
  totp: &TOTP,
) -> Result<TwoFactor, ApiError> {
  let data: TwoFactor = TwoFactor {
    setup: None,
    recovery_code: crate::cipher::two_factor::generate_recovery_code(),
    secret_base32: totp.get_secret_base32(),
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
  let update_result = collection.update_one(query, update, None).await?;

  if update_result.modified_count == 0 {
    return Err(ApiError::new(
      Status::InternalServerError,
      "Error while updating user 2FA in database".to_string(),
    ));
  }
  Ok(data)
}

pub(crate) async fn update_2fa_state_in_db(
  db: &State<Database>,
  user: &User,
  enable: bool,
) -> Result<(), ApiError> {
  let mongodb_client = db.inner();
  let collection: Collection<User> =
    mongodb_client.collection(USER_COLLECTION_NAME);

  let query = doc! {
    "_id": user.id.clone()
  };

  let update = doc! {
    "$set": {
      "twoFactor.setup": if enable { Some(bson::DateTime::now()) } else { None }
    }
  };
  let update_result = collection.update_one(query, update, None).await?;

  if update_result.modified_count == 0 {
    return Err(ApiError::new(
      Status::InternalServerError,
      "Error while updating user 2FA in database".to_string(),
    ));
  }
  Ok(())
}

pub(crate) async fn delete_2fa_in_db(
  db: &State<Database>,
  user_id: &ObjectId,
) -> Result<(), ApiError> {
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
  let update_result = collection.update_one(query, update, None).await?;
  if update_result.modified_count == 0 {
    return Err(ApiError::new(
      Status::InternalServerError,
      "Error while disabling user 2FA in database".to_string(),
    ));
  }
  Ok(())
}
