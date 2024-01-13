use crate::db::{CloudProvider, ToCollectionName};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

const ORGANIZATION_PROJECT_CLUSTER_COLLECTION_NAME: &str =
  "organizationProjectClusters";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ClusterStatus {
  #[serde(rename = "CREATING")]
  Creating,

  #[serde(rename = "RUNNING")]
  Running,

  #[serde(rename = "DELETING")]
  Deleting,

  #[serde(rename = "ERROR")]
  Error,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationProjectCluster {
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

  #[serde(rename = "cloudProvider")]
  pub cloud_provider: CloudProvider,

  #[serde(rename = "credential_id")]
  pub credential_id: ObjectId,

  #[serde(rename = "status")]
  pub status: ClusterStatus,
}

impl ToCollectionName for OrganizationProjectCluster {
  fn collection_name() -> String {
    String::from(ORGANIZATION_PROJECT_CLUSTER_COLLECTION_NAME)
  }
}

impl OrganizationProjectCluster {
  pub fn new(
    organization_id: ObjectId,
    project_id: ObjectId,
    name: String,
    description: Option<String>,
    cloud_provider: CloudProvider,
    credential_id: ObjectId,
    status: ClusterStatus,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      organization_id,
      project_id,
      name,
      description,
      cloud_provider,
      credential_id,
      status,
    }
  }
}
