use crate::db::query::organization_invitation::OrganizationInvitationQuery;
use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::oid::ObjectId;
use bson::{doc, Bson};
use chrono::{DateTime, Utc};
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::db::organization_member::OrganizationMember;
use crate::db::query::cursor_to_vec;

const ORGANIZATION_INVITATION_COLLECTION_NAME: &str = "organizationInvitations";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationInvitation {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "senderId")]
  pub sender_id: ObjectId,

  #[serde(rename = "receiverId")]
  pub receiver_id: ObjectId,

  #[serde(rename = "status")]
  pub status: InvitationStatus,

  #[serde(rename = "responseAt")]
  pub response_at: Option<DateTime<Utc>>,

  #[serde(rename = "role")]
  pub role: ObjectId,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum InvitationStatus {
  #[serde(rename = "PENDING")]
  Pending,

  #[serde(rename = "ACCEPTED")]
  Accepted,

  #[serde(rename = "REJECTED")]
  Rejected,
}

impl Display for InvitationStatus {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let str = match self {
      InvitationStatus::Pending => "PENDING".to_string(),
      InvitationStatus::Accepted => "ACCEPTED".to_string(),
      InvitationStatus::Rejected => "REJECTED".to_string(),
    };
    write!(f, "{}", str)
  }
}

impl OrganizationInvitation {
  pub fn new(
    organization_id: ObjectId,
    sender_id: ObjectId,
    receiver_id: ObjectId,
    role: ObjectId,
  ) -> Self {
    let id = ObjectId::new();
    let status = InvitationStatus::Pending;
    let response_at = None;
    OrganizationInvitation {
      id,
      organization_id,
      sender_id,
      receiver_id,
      status,
      response_at,
      role,
    }
  }
}

impl ToCollectionName for OrganizationInvitation {
  fn collection_name() -> String {
    String::from(ORGANIZATION_INVITATION_COLLECTION_NAME)
  }
}

impl CommonCollection<OrganizationInvitation> {
  pub async fn get_of_user_of_org(
    &self,
    user_id: &ObjectId,
    org_id: &ObjectId,
  ) -> CommonResult<Option<OrganizationInvitation>> {
    let filter = doc! {
      "receiverId": user_id,
      "organizationId": org_id,
    };
    let result = self.collection.find_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn get_with_role(
    &self,
    org_id: &ObjectId,
    role: &ObjectId,
  ) -> CommonResult<Vec<OrganizationInvitation>> {
    let filter = doc! { "organizationId": org_id, "role": role };
    let result = self.collection.find(filter, None).await?;
    let result = cursor_to_vec(result).await?;
    return Ok(result);
  }
  
  pub async fn update_status(
    &self,
    invitation_id: &ObjectId,
    status: &InvitationStatus,
  ) -> CommonResult<UpdateResult> {
    let response_at: Bson = Bson::DateTime(bson::DateTime::now());
    let status_str = status.to_string();
    let filter = doc! {
      "_id": invitation_id,
    };
    let update = doc! {
      "$set": {
        "status": status_str,
        "responseAt": response_at,
      },
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }
}
