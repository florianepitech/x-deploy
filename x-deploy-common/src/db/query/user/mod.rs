use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Database};

pub mod email;
pub mod password;
mod phone;
pub mod two_factor;

impl User {
  pub async fn insert(
    &self,
    db: &Database,
  ) -> CommonResult<InsertOneResult> {
    let collection: Collection<Self> = db.collection(USER_COLLECTION_NAME);
    let result = collection.insert_one(self, None).await?;
    return Ok(result);
  }

  pub async fn find_with_id(
    db: &Database,
    id: &ObjectId,
  ) -> CommonResult<Option<Self>> {
    let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
    let user = collection
      .find_one(
        doc! {
          "_id": id
        },
        None,
      )
      .await?;
    return Ok(user);
  }

  pub async fn find_with_email(
    db: &Database,
    email: &String,
  ) -> CommonResult<Option<Self>> {
    let collection: Collection<User> = db.collection(USER_COLLECTION_NAME);
    let user = collection
      .find_one(
        doc! {
          "email.email": email
        },
        None,
      )
      .await?;
    return Ok(user);
  }
}
