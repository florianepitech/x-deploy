use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::CommonResult;
use bson::{doc, oid};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Collection, Database};
use oid::ObjectId;

impl Organization {
  pub async fn insert(
    &self,
    db: &Database,
  ) -> CommonResult<InsertOneResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_COLLECTION_NAME);
    let result = collection.insert_one(self, None).await?;
    return Ok(result);
  }

  pub async fn delete(
    &self,
    db: &Database,
  ) -> CommonResult<DeleteResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_COLLECTION_NAME);
    let filter = doc! {
      "_id": &self.id,
    };
    let result = collection.delete_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn update(
    &self,
    db: &Database,
  ) -> CommonResult<UpdateResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_COLLECTION_NAME);
    let filter = doc! {
      "_id": &self.id,
    };
    let result = collection.replace_one(filter, self, None).await?;
    return Ok(result);
  }
}
