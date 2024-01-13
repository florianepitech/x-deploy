use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, UpdateResult};
use serde::{Deserialize, Serialize};

const ORGANIZATION_CREDENTIAL_OVH_COLLECTION_NAME: &str =
  "organizationsCredentialsOvh";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationCredentialOvh {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "applicationKey")]
  pub application_key: String,

  #[serde(rename = "applicationSecret")]
  pub application_secret: String,

  #[serde(rename = "consumerKey")]
  pub consumer_key: String,
}

impl OrganizationCredentialOvh {
  pub fn new(
    organization_id: ObjectId,
    name: String,
    description: Option<String>,
    application_key: String,
    application_secret: String,
    consumer_key: String,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      organization_id,
      name,
      description,
      application_key,
      application_secret,
      consumer_key,
    }
  }
}

impl ToCollectionName for OrganizationCredentialOvh {
  fn collection_name() -> String {
    String::from(ORGANIZATION_CREDENTIAL_OVH_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationCredentialOvh> {
  pub async fn get_all_of_org(
    &self,
    organization_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationCredentialOvh>> {
    let filter = doc! {
      "organizationId": organization_id
    };
    let result = self.collection.find(filter, None).await?;
    let result = cursor_to_vec(result).await?;
    return Ok(result);
  }

  pub async fn get_by_id_and_org_id(
    &self,
    id: &ObjectId,
    organization_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationCredentialOvh>> {
    let filter = doc! {
      "_id": id,
      "organizationId": organization_id
    };
    let result = self.collection.find_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn delete_by_id_and_org_id(
    &self,
    id: &ObjectId,
    organization_id: &ObjectId,
  ) -> CommonResult<DeleteResult> {
    let filter = doc! {
      "_id": id,
      "organizationId": organization_id
    };
    let result = self.collection.delete_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn update_info(
    &self,
    id: &ObjectId,
    organization_id: &ObjectId,
    name: &String,
    description: &Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id,
      "organizationId": organization_id
    };
    let bson_description = match description {
      Some(description) => bson::Bson::String(description.clone()),
      None => bson::Bson::Null,
    };
    let update = doc! {
      "$set": {
        "name": name,
        "description": bson_description,
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }
}
