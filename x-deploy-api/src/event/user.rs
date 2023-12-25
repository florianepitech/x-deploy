use crate::event::send_event;
use kafka::Error;
use serde_json::{json, Value};

pub(super) const USER_REGISTERED_TOPIC: &str = "user.registered";

pub(crate) fn send_user_registered_event(
  object_id: bson::oid::ObjectId,
  firstname: String,
  lastname: String,
  email: String,
) -> Result<(), Error> {
  let json: Value = json!({
    "id": object_id.to_string(),
    "firstname": firstname,
    "lastname": lastname,
    "email": email,
  });
  send_event(USER_REGISTERED_TOPIC.to_string(), json)
}
