use serde::{Deserialize, Serialize};

mod auth;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    message: String,
}