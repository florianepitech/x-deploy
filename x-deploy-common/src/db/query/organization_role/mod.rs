use crate::db::organization_role::{
  OrganizationRole, ORGANIZATION_ROLE_COLLECTION_NAME,
};
use crate::db::query::cursor_to_vec;
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Database};

impl OrganizationRole {
  pub async fn insert(
    &self,
    db: &Database,
  ) -> CommonResult<InsertOneResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_ROLE_COLLECTION_NAME);
    let result = collection.insert_one(self, None).await?;
    return Ok(result);
  }

  pub async fn get_all_of_org(
    db: &Database,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<Self>> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_ROLE_COLLECTION_NAME);
    let filter = doc! {
      "organizationId": org_id,
    };
    let cursor = collection.find(filter, None).await?;
    let result = cursor_to_vec(cursor).await?;
    return Ok(result);
  }

  pub async fn get_of_org(
    db: &Database,
    org_id: &ObjectId,
    role_id: &ObjectId,
  ) -> CommonResult<Option<Self>> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_ROLE_COLLECTION_NAME);
    let filter = doc! {
      "_id": role_id,
      "organizationId": org_id,
    };
    let result = collection.find_one(filter, None).await?;
    return Ok(result);
  }
}
