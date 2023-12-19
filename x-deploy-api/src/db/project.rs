use bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

pub(crate) const PROJECT_COLLECTION_NAME: &str = "projects";

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

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
      organization_id,
    }
  }
}
