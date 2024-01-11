use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const PROJECT_COLLECTION_NAME: &str = "projects";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Project {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "logoUrl")]
  pub logo_url: Option<String>,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,
}

impl Project {
  pub fn new(
    name: String,
    description: String,
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
