use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const ORGANIZATION_COLLECTION_NAME: &str = "organizations";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Organization {
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
}

impl Organization {
  pub fn new(
    name: String,
    description: String,
    website: String,
    contact_email: String,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      name,
      description,
      logo_url: None,
      website,
      contact_email,
    }
  }
}
