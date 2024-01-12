use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::oid::ObjectId;
use bson::{doc, Bson};
use mongodb::results::UpdateResult;
use serde::{Deserialize, Serialize};

const PROJECT_COLLECTION_NAME: &str = "organizationProjects";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationProject {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "logoUrl")]
  pub logo_url: Option<String>,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,
}

impl OrganizationProject {
  pub fn new(
    name: String,
    description: Option<String>,
    organization_id: ObjectId,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      name,
      description,
      logo_url: None,
      organization_id,
    }
  }
}

impl ToCollectionName for OrganizationProject {
  fn collection_name() -> String {
    String::from(PROJECT_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationProject> {
  pub async fn get_of_org(
    &self,
    org_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationProject>> {
    let filter = doc! {
      "organizationId": org_id,
    };
    let cursor = self.collection.find(filter, None).await?;
    let projects = cursor_to_vec(cursor).await?;
    Ok(projects)
  }

  pub async fn get_with_id_of_org(
    &self,
    project_id: &ObjectId,
    org_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationProject>> {
    let filter = doc! {
      "_id": project_id,
      "organizationId": org_id,
    };
    let project = self.collection.find_one(filter, None).await?;
    Ok(project)
  }

  pub async fn update_info(
    &self,
    project_id: &ObjectId,
    name: &String,
    description: &Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": project_id
    };
    let description_bson = match description {
      Some(desc) => Bson::String(desc.clone()),
      None => Bson::Null,
    };
    let update = doc! {
      "$set": {
        "name": name,
        "description": description_bson
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }

  pub async fn update_logo_url(
    &self,
    project_id: &ObjectId,
    logo_url: &Option<String>,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": project_id
    };
    let bson_logo = match logo_url {
      Some(url) => Bson::String(url.clone()),
      None => Bson::Null,
    };
    let update = doc! {
      "$set": {
        "logoUrl": bson_logo
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    Ok(result)
  }
}
