use crate::event::send_event;
use crate::CommonResult;
use bson::oid;
use oid::ObjectId;
use serde_json::{json, Value};

// Organization created

pub const ORGANIZATION_CREATED_TOPIC: &str = "organization.created";

pub fn send_organization_created_event(
  kafka_url: Vec<String>,
  user_id: ObjectId,
  organization_id: ObjectId,
) -> CommonResult<()> {
  let json: Value = json!({
    "userId": user_id.to_string(),
    "organizationId": organization_id.to_string(),
  });
  send_event(kafka_url, ORGANIZATION_CREATED_TOPIC.to_string(), json)
}

// Organization deleted

pub const ORGANIZATION_DELETED_TOPIC: &str = "organization.deleted";

#[deprecated]
pub fn send_organization_deleted_event(
  kafka_url: Vec<String>,
  user_id: ObjectId,
  organization_id: ObjectId,
) -> CommonResult<()> {
  let json: Value = json!({
    "userId": user_id.to_string(),
    "organizationId": organization_id.to_string(),
  });
  send_event(kafka_url, ORGANIZATION_DELETED_TOPIC.to_string(), json)
}

// Organization transfer ownership

pub const ORGANIZATION_TRANSFER_OWNERSHIP_TOPIC: &str =
  "organization.transfer_ownership";

pub fn send_organization_transfer_ownership_event(
  kafka_url: Vec<String>,
  organization_id: ObjectId,
  previous_owner_id: ObjectId,
  new_owner_id: ObjectId,
) -> CommonResult<()> {
  let json: Value = json!({
    "organizationId": organization_id.to_string(),
    "previousOwnerId": previous_owner_id.to_string(),
    "newOwnerId": new_owner_id.to_string(),
  });
  send_event(
    kafka_url,
    ORGANIZATION_TRANSFER_OWNERSHIP_TOPIC.to_string(),
    json,
  )
}
