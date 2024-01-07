use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::CommonResult;
use bson::{doc, Bson};
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};

impl User {
  pub async fn two_factor_update(
    &self,
    db: &Database,
  ) -> CommonResult<UpdateResult> {
    let collection: Collection<Self> = db.collection(USER_COLLECTION_NAME);
    let new_value = match &self.two_factor {
      Some(two_factor) => bson::to_bson(two_factor)?,
      None => Bson::Null,
    };
    let filter = doc! {
      "_id": self.id.clone()
    };
    let update = doc! {
      "$set": {
        "twoFactor": new_value
      }
    };
    let result = collection.update_one(filter, update, None).await?;
    return Ok(result);
  }
}
