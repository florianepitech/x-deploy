use crate::db::ToCollectionName;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

const ORGANIZATION_PROJECT_DEPLOYMENT_COLLECTION_NAME: &str =
  "organizationProjectDeployments";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationProjectDeployment {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "imageName")]
  pub image_name: String,

  #[serde(rename = "imageTag")]
  pub image_tag: String,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "projectId")]
  pub project_id: ObjectId,

  #[serde(rename = "clusterId")]
  pub cluster_id: ObjectId,
}

impl ToCollectionName for OrganizationProjectDeployment {
  fn collection_name() -> String {
    String::from(ORGANIZATION_PROJECT_DEPLOYMENT_COLLECTION_NAME)
  }
}
