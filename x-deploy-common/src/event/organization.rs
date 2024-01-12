use crate::event::ToTopicName;
use bson::oid;
use serde::{Deserialize, Serialize};

// Organization created

pub const ORGANIZATION_CREATED_TOPIC: &str = "organization.created";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationCreatedEvent {
  pub id: String,
  pub name: String,
  pub description: Option<String>,
  pub creator_id: String,
  pub creator_firstname: String,
  pub creator_lastname: String,
  pub creator_email: String,
}

impl ToTopicName for OrganizationCreatedEvent {
  fn topic_name() -> String {
    ORGANIZATION_CREATED_TOPIC.to_string()
  }
}

// Organization deleted

pub const ORGANIZATION_DELETED_TOPIC: &str = "organization.deleted";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationDeletedEvent {
  pub id: String,
  pub name: String,
  pub description: String,
  pub deleter_id: String,
  pub deleter_firstname: String,
  pub deleter_lastname: String,
}

impl ToTopicName for OrganizationDeletedEvent {
  fn topic_name() -> String {
    ORGANIZATION_DELETED_TOPIC.to_string()
  }
}

// Organization transfer ownership

pub const ORGANIZATION_TRANSFER_OWNERSHIP_TOPIC: &str =
  "organization.transfer_ownership";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationTransferOwnershipEvent {
  pub id: String,
  pub name: String,
  pub description: String,
  pub old_owner_id: String,
  pub old_owner_firstname: String,
  pub old_owner_lastname: String,
  pub new_owner_id: String,
  pub new_owner_firstname: String,
  pub new_owner_lastname: String,
}

impl ToTopicName for OrganizationTransferOwnershipEvent {
  fn topic_name() -> String {
    ORGANIZATION_TRANSFER_OWNERSHIP_TOPIC.to_string()
  }
}
