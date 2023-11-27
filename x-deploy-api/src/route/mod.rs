use serde::{Deserialize, Serialize};

mod auth;
pub(crate) mod deploy;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    message: String,
}