use crate::db::organization_project::OrganizationProject;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[deprecated]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationQuery {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "logoUrl")]
  pub logo_url: Option<String>,

  #[serde(rename = "website")]
  pub website: String,

  #[serde(rename = "contactEmail")]
  pub contact_email: String,

  #[serde(rename = "projects")]
  pub projects: Vec<OrganizationProject>,
}
