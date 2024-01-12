use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::UpdateResult;
use serde::{Deserialize, Serialize};

const ORGANIZATION_APIKEY_COLLECTION_NAME: &str = "organizations";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationApiKey {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "key")]
  pub key: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "expiresAt")]
  pub expires_at: i64,
}

impl OrganizationApiKey {
  pub fn new(
    organization_id: ObjectId,
    key: String,
    description: String,
    expires_at: i64,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      organization_id,
      key,
      description,
      expires_at,
    }
  }
}

impl ToCollectionName for OrganizationApiKey {
  fn collection_name() -> String {
    String::from(ORGANIZATION_APIKEY_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationApiKey> {
  pub async fn update_description(
    &self,
    id: &ObjectId,
    description: &String,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id,
    };
    let update = doc! {
      "$set": {
        "description": description,
      },
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }
}
