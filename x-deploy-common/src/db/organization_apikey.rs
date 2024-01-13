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

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "key")]
  pub key: String,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "roleId")]
  pub role_id: Option<ObjectId>,

  #[serde(rename = "expiresAt")]
  pub expires_at: Option<bson::DateTime>,
}

impl OrganizationApiKey {
  pub fn new(
    name: String,
    description: Option<String>,
    key: String,
    organization_id: ObjectId,
    role_id: Option<ObjectId>,
    expires_at: Option<bson::DateTime>,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      name,
      organization_id,
      key,
      description,
      role_id,
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
  pub async fn update_info(
    &self,
    id: &ObjectId,
    name: &String,
    description: &Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id,
    };
    let bson_description = match description {
      Some(description) => bson::to_bson(description)?,
      None => bson::Bson::Null,
    };
    let update = doc! {
      "$set": {
        "name": name,
        "description": bson_description,
      },
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }
}
