use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const ORGANIZATION_CREDENTIAL_DOCKER_HUB_COLLECTION_NAME: &str =
  "organizationsCredentialsDockerHub";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationCredentialDockerHub {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "name")]
  pub name: String,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "accessToken")]
  pub access_token: String,
}

impl OrganizationCredentialDockerHub {
  pub fn new(
    organization_id: ObjectId,
    name: String,
    description: String,
    access_token: String,
  ) -> Self {
    Self {
      id: ObjectId::new(),
      organization_id,
      name,
      description,
      access_token,
    }
  }
}
