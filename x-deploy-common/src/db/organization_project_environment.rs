use crate::db::ToCollectionName;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

const ORGANIZATION_PROJECT_ENVIRONMENT_COLLECTION_NAME: &str =
  "organizationProjectEnvironments";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationProjectEnvironment {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: Option<String>,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "projectId")]
  pub project_id: ObjectId,

  #[serde(rename = "clusterId")]
  pub cluster_id: ObjectId,
}

impl OrganizationProjectEnvironment {
  pub fn new(
    organization_id: ObjectId,
    project_id: ObjectId,
    name: String,
    description: Option<String>,
    cluster_id: ObjectId,
  ) -> Self {
    OrganizationProjectEnvironment {
      id: ObjectId::new(),
      organization_id,
      project_id,
      name,
      description,
      cluster_id,
    }
  }
}

impl ToCollectionName for OrganizationProjectEnvironment {
  fn collection_name() -> String {
    String::from(ORGANIZATION_PROJECT_ENVIRONMENT_COLLECTION_NAME)
  }
}
