use crate::db::organization_role::OrganizationRole;
use crate::db::query::cursor_to_vec;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::results::DeleteResult;
use serde::{Deserialize, Serialize};

const ORGANIZATION_MEMBER_COLLECTION_NAME: &str = "organizationMembers";

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

impl ToCollectionName for OrganizationMember {
  fn collection_name() -> String {
    String::from(ORGANIZATION_MEMBER_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationMember> {
  pub async fn get_with_role(
    &self,
    org_id: &ObjectId,
    role: &ObjectId,
  ) -> CommonResult<Vec<OrganizationMember>> {
    let filter = doc! { "organization": org_id, "role": role };
    let result = self.collection.find(filter, None).await?;
    let result = cursor_to_vec(result).await?;
    return Ok(result);
  }

  pub async fn delete_by_id_and_org(
    &self,
    id: &ObjectId,
    org_id: &ObjectId,
  ) -> CommonResult<DeleteResult> {
    let filter = doc! {
      "_id": id,
      "organizationId": org_id,
    };
    let result = self.collection.delete_one(filter, None).await?;
    Ok(result)
  }
}
