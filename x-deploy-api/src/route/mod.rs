use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub(crate) mod deploy;

pub mod auth;
pub mod organization;
pub mod ovh;
pub mod project;
pub mod account;

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

pub(crate) type MessageResult = Result<Json<Message>, Custom<Json<Message>>>;

pub(crate) type CustomResult<T> = Result<T, Custom<Json<Message>>>;

#[macro_export]
macro_rules! custom_response {
    ($status:expr, $msg:expr) => {
        Err(Custom(
            $status,
            Json(Message {
                message: $msg.to_string(),
            }),
        ))
    };
}

#[macro_export]
macro_rules! custom_message {
    ($status:expr, $msg:expr) => {
        Ok(Custom(
            $status,
            Json(Message {
                message: $msg.to_string(),
            }),
        ))
    };
}
