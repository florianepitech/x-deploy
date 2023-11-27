use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    #[serde(rename = "message")]
    message: String,
}