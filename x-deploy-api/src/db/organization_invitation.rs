use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

pub(crate) const ORGANIZATION_INVITATION_COLLECTION_NAME: &str =
  "organizationInvitations";

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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum InvitationStatus {
  #[serde(rename = "PENDING")]
  Pending,

  #[serde(rename = "ACCEPTED")]
  Accepted,

  #[serde(rename = "REJECTED")]
  Rejected,
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
