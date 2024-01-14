use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl Display for ClusterStatus {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      ClusterStatus::Creating => write!(f, "CREATING"),
      ClusterStatus::Running => write!(f, "RUNNING"),
      ClusterStatus::Deleting => write!(f, "DELETING"),
      ClusterStatus::Error => write!(f, "ERROR"),
    }
  }
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
  pub cloud_provider: String,

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
    cloud_provider: String,
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

impl CommonCollection<OrganizationProjectCluster> {
  pub async fn get_of_org_and_project(
    &self,
    org_id: &ObjectId,
    project_id: &ObjectId,
  ) -> CommonResult<Vec<OrganizationProjectCluster>> {
    let filter = doc! {
      "organizationId": org_id,
      "projectId": project_id,
    };
    let cursor = self.collection.find(filter, None).await?;
    let clusters = cursor_to_vec(cursor).await?;
    Ok(clusters)
  }

  pub async fn get_with_id_of_project(
    &self,
    org_id: &ObjectId,
    project_id: &ObjectId,
    cluster_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationProjectCluster>> {
    let filter = doc! {
      "_id": cluster_id,
      "organizationId": org_id,
      "projectId": project_id,
    };
    let cluster = self.collection.find_one(filter, None).await?;
    Ok(cluster)
  }
}
