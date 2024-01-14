use crate::db::user::User;
use crate::CommonResult;
use bson::{doc, oid};
use oid::ObjectId;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod organization;
pub mod organization_apikey;
pub mod organization_credential_aws;
pub mod organization_credential_docker_hub;
pub mod organization_credential_ovh;
pub mod organization_invitation;
pub mod organization_member;
pub mod organization_project;
pub mod organization_project_cluster;
pub mod organization_project_environment;
pub mod organization_role;
pub mod query;
pub mod user;
mod organization_project_deployment;

pub trait ToCollectionName {
  fn collection_name() -> String;
}

pub struct CommonCollection<T>
where
  T: ToCollectionName + DeserializeOwned + Serialize + Unpin + Send + Sync,
{
  pub collection: mongodb::Collection<T>,
}

impl<T> CommonCollection<T>
where
  T: ToCollectionName + DeserializeOwned + Serialize + Unpin + Send + Sync,
{
  pub fn new(db: &mongodb::Database) -> CommonCollection<T> {
    let collection = db.collection(T::collection_name().as_str());
    Self { collection }
  }

  pub async fn insert_one(
    &self,
    document: &T,
  ) -> CommonResult<mongodb::results::InsertOneResult> {
    let result = self.collection.insert_one(document, None).await?;
    Ok(result)
  }

  pub async fn insert_many(
    &self,
    documents: &Vec<T>,
  ) -> CommonResult<mongodb::results::InsertManyResult> {
    let result = self.collection.insert_many(documents, None).await?;
    Ok(result)
  }

  pub async fn get_by_id(
    &self,
    object_id: &ObjectId,
  ) -> CommonResult<Option<T>> {
    let filter = doc! { "_id": object_id };
    let result = self.collection.find_one(filter, None).await?;
    Ok(result)
  }

  pub async fn delete_by_id(
    &self,
    object_id: &ObjectId,
  ) -> CommonResult<mongodb::results::DeleteResult> {
    let filter = doc! { "_id": object_id };
    let result = self.collection.delete_one(filter, None).await?;
    Ok(result)
  }
}
