use crate::db::organization_credential_aws::{
  OrganizationCredentialAws, ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME,
};
use crate::db::query::cursor_to_vec;
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Collection, Cursor, Database};

impl OrganizationCredentialAws {
  pub async fn insert(
    &self,
    db: &Database,
  ) -> CommonResult<InsertOneResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME);
    let result = collection.insert_one(self, None).await?;
    return Ok(result);
  }

  pub async fn delete(
    &self,
    db: &Database,
  ) -> CommonResult<DeleteResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME);
    let filter = doc! {
      "_id": &self.id,
      "organizationId": &self.organization_id
    };
    let result = collection.delete_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn update(
    &self,
    db: &Database,
  ) -> CommonResult<UpdateResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME);
    let filter = doc! {
      "_id": &self.id,
      "organizationId": &self.organization_id
    };
    let result = collection.replace_one(filter, self, None).await?;
    return Ok(result);
  }

  pub async fn delete_by_id(
    db: &Database,
    org_id: &ObjectId,
    id: &ObjectId,
  ) -> CommonResult<DeleteResult> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME);
    let filter = doc! {
      "_id": id,
      "organizationId": org_id
    };
    let result = collection.delete_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn find_by_id(
    db: &Database,
    org_id: &ObjectId,
    id: &ObjectId,
  ) -> CommonResult<Option<Self>> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME);
    let filter = doc! {
      "_id": id,
      "organizationId": org_id
    };
    let result = collection.find_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn find_all_for_org(
    db: &Database,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<Self>> {
    let collection: Collection<Self> =
      db.collection(ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME);
    let filter = doc! {
      "organizationId": org_id
    };
    let result_query: Cursor<Self> = collection.find(filter, None).await?;
    let result: Vec<Self> = cursor_to_vec(result_query).await?;
    return Ok(result);
  }
}
