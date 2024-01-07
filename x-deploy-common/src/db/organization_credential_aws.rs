use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const ORGANIZATION_CREDENTIAL_AWS_COLLECTION_NAME: &str =
  "organizationsCredentialsAws";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationCredentialAws {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "access_key")]
  pub access_key: String,

  #[serde(rename = "secret_key")]
  pub secret_key: String,
}

impl OrganizationCredentialAws {
  pub fn new(
    organization_id: ObjectId,
    name: String,
    description: String,
    access_key: String,
    secret_key: String,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      organization_id,
      name,
      description,
      access_key,
      secret_key,
    }
  }
}
