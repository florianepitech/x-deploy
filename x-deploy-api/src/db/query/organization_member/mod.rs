use crate::db::organization_custom_role::{
  OrganizationRole, ORGANIZATION_ROLE_COLLECTION_NAME,
};
use crate::db::organization_member::{
  OrganizationMember, ORGANIZATION_MEMBER_COLLECTION_NAME,
};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::error::ApiError;
use bson::{doc, oid, Document};
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Cursor, Database};
use oid::ObjectId;
use rocket::futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::iter::Filter;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OrganizationMemberQuery {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "organizationId")]
  pub organization_id: ObjectId,

  #[serde(rename = "user")]
  pub user: User,

  #[serde(rename = "isOwner")]
  pub is_owner: bool,

  #[serde(rename = "role")]
  pub role: Option<OrganizationRole>,
}

pub async fn query_organization_member_insert_one(
  db: &Database,
  member: &OrganizationMember,
) -> Result<InsertOneResult, ApiError> {
  let collection: Collection<OrganizationMember> =
    db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);
  let result = collection.insert_one(member, None).await?;
  return Ok(result);
}

pub async fn query_organization_member_get_all_in_org(
  db: &Database,
  org_id: &ObjectId,
) -> Result<Vec<OrganizationMemberQuery>, ApiError> {
  let collection: Collection<OrganizationMember> =
    db.collection(ORGANIZATION_MEMBER_COLLECTION_NAME);

  let filter = doc! {
    "$match": {
      "organizationId": org_id
    }
  };

  let mut pipeline = default_pipeline();
  pipeline.insert(0, filter);

  let mut cursor = collection.aggregate(pipeline, None).await?;
  let mut result: Vec<OrganizationMemberQuery> = Vec::new();
  while let Some(doc) = cursor.next().await {
    let result_doc = doc?;
    let result_doc: OrganizationMemberQuery = bson::from_document(result_doc)?;
    result.push(result_doc);
  }
  return Ok(result);
}

fn default_pipeline() -> Vec<Document> {
  return vec![
    doc! {
      "$lookup": {
        "from": USER_COLLECTION_NAME,
        "localField": "userId",
        "foreignField": "_id",
        "as": "user"
      }
    },
    doc! {
      "$lookup": {
        "from": ORGANIZATION_ROLE_COLLECTION_NAME,
        "localField": "role",
        "foreignField": "_id",
        "as": "role"
      }
    },
    doc! {
      "$unwind": "$user"
    },
    doc! {
      "$unwind": {
        "path": "$role",
        "preserveNullAndEmptyArrays": true
      }
    },
  ];
}
