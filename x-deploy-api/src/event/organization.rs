use crate::event::send_event;
use bson::oid;
use kafka::Error;
use oid::ObjectId;
use serde_json::{json, Value};

// Organization created

pub(super) const ORGANIZATION_CREATED_TOPIC: &str = "organization.created";

pub(crate) fn send_organization_created_event(
  user_id: ObjectId,
  organization_id: ObjectId,
) -> Result<(), Error> {
  let json: Value = json!({
    "userId": user_id.to_string(),
    "organizationId": organization_id.to_string(),
  });
  send_event(ORGANIZATION_CREATED_TOPIC.to_string(), json)
}

// Organization deleted

pub(super) const ORGANIZATION_DELETED_TOPIC: &str = "organization.deleted";

#[deprecated]
pub(crate) fn send_organization_deleted_event(
  user_id: ObjectId,
  organization_id: ObjectId,
) -> Result<(), Error> {
  let json: Value = json!({
    "userId": user_id.to_string(),
    "organizationId": organization_id.to_string(),
  });
  send_event(ORGANIZATION_DELETED_TOPIC.to_string(), json)
}

// Organization transfer ownership

pub(super) const ORGANIZATION_TRANSFER_OWNERSHIP_TOPIC: &str =
  "organization.transfer_ownership";

pub(crate) fn send_organization_transfer_ownership_event(
  organization_id: ObjectId,
  previous_owner_id: ObjectId,
  new_owner_id: ObjectId,
) -> Result<(), Error> {
  let json: Value = json!({
    "organizationId": organization_id.to_string(),
    "previousOwnerId": previous_owner_id.to_string(),
    "newOwnerId": new_owner_id.to_string(),
  });
  send_event(ORGANIZATION_TRANSFER_OWNERSHIP_TOPIC.to_string(), json)
}
