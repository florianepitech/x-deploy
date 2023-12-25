use crate::event::send_event;
use kafka::Error;
use serde_json::{json, Value};

pub(super) const USER_REGISTERED_TOPIC: &str = "user.registered";

pub(crate) fn user_registered_event(
  object_id: bson::oid::ObjectId
) -> Result<(), Error> {
  let json: Value = json!({
    "id": object_id.to_string(),
  });
  send_event(USER_REGISTERED_TOPIC.to_string(), json)
}
