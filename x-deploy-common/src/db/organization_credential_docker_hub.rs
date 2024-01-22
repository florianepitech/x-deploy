use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::{Bson, doc};
use bson::oid::ObjectId;
use mongodb::results::{DeleteResult, UpdateResult};
use serde::{Deserialize, Serialize};

const ORGANIZATION_CREDENTIAL_DOCKER_HUB_COLLECTION_NAME: &str =
  "organizationsCredentialsDockerHub";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationCredentialDockerHub {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "accessToken")]
  pub access_token: String,
}

impl OrganizationCredentialDockerHub {
  pub fn new(
    organization_id: ObjectId,
    name: String,
    description: Option<String>,
    access_token: String,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      organization_id,
      name,
      description,
      access_token,
    }
  }
}

impl ToCollectionName for OrganizationCredentialDockerHub {
  fn collection_name() -> String {
    String::from(ORGANIZATION_CREDENTIAL_DOCKER_HUB_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationCredentialDockerHub> {
  pub async fn get_all_of_org(
    &self,
    organization_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationCredentialDockerHub>> {
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
  ) -> CommonResult<Option<OrganizationCredentialDockerHub>> {
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
      Some(description) => Bson::String(description.clone()),
      None => Bson::Null,
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

  pub async fn delete_of_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<DeleteResult> {
    let filter = doc! {
      "organizationId": org_id,
    };
    let result = self.collection.delete_many(filter, None).await?;
    Ok(result)
  }
}
