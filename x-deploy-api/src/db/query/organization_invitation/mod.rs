use crate::db::organization_custom_role::OrganizationRole;
use crate::db::organization_invitation::{
  InvitationStatus, ORGANIZATION_INVITATION_COLLECTION_NAME,
};
use crate::db::user::User;
use crate::error::ApiError;
use bson::oid::ObjectId;
use bson::{doc, Document};
use chrono::{DateTime, Utc};
use mongodb::{Collection, Database};
use rocket::futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationInvitationQuery {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "senderId")]
  pub sender_id: User,

  #[serde(rename = "receiverId")]
  pub receiver_id: User,

  #[serde(rename = "status")]
  pub status: InvitationStatus,

  #[serde(rename = "responseAt")]
  pub response_at: Option<DateTime<Utc>>,

  #[serde(rename = "role")]
  pub role: OrganizationRole,
}

pub async fn query_organization_invitation_get_all(
  db: &Database,
  organization_id: &ObjectId,
) -> Result<Vec<OrganizationInvitationQuery>, ApiError> {
  let collection: Collection<OrganizationInvitationQuery> =
    db.collection(ORGANIZATION_INVITATION_COLLECTION_NAME);
  let filter = doc! {
    "organizationId": organization_id,
  };
  let mut pipeline = default_pipeline();
  pipeline.insert(
    0,
    doc! {
      "$match": filter,
    },
  );
  let mut cursor = collection.aggregate(pipeline, None).await?;
  let mut result: Vec<OrganizationInvitationQuery> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let doc: OrganizationInvitationQuery = bson::from_document(doc?)?;
    result.push(doc);
  }
  return Ok(result);
}

// Private functions

fn default_pipeline() -> Vec<Document> {
  vec![
    doc! {
      "$lookup": {
        "from": "users",
        "localField": "senderId",
        "foreignField": "_id",
        "as": "sender",
      },
    },
    doc! {
      "$lookup": {
        "from": "users",
        "localField": "receiverId",
        "foreignField": "_id",
        "as": "receiver",
      },
    },
    doc! {
      "$lookup": {
        "from": "organizationRoles",
        "localField": "role",
        "foreignField": "_id",
        "as": "role",
      },
    },
    doc! {
      "$unwind": "$sender",
    },
    doc! {
      "$unwind": "$receiver",
    },
    doc! {
      "$unwind": "$role",
    },
  ]
}
