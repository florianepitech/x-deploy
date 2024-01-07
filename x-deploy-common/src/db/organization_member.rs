use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub const ORGANIZATION_MEMBER_COLLECTION_NAME: &str = "organizationMembers";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationMember {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "userId")]
  pub user_id: ObjectId,

  #[serde(rename = "isOwner")]
  pub is_owner: bool,

  #[serde(rename = "role")]
  pub role: Option<ObjectId>,
}

impl OrganizationMember {
  pub fn new(
    organization_id: ObjectId,
    user_id: ObjectId,
    is_owner: bool,
    role: Option<ObjectId>,
  ) -> Self {
    let id = ObjectId::new();
    OrganizationMember {
      id,
      organization_id,
      user_id,
      is_owner,
      role,
    }
  }
}
