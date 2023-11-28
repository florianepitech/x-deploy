use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub(crate) mod deploy;

pub mod auth;
pub mod ovh;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub(crate) struct Message {
    #[serde(rename = "message")]
    pub(crate) message: String,
}

impl Message {
    pub(crate) fn new(message: String) -> Self {
        Self { message }
    }
}
