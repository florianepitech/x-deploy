use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
pub(crate) mod deploy;

pub mod account;
pub mod auth;
pub mod organization;
pub mod ovh;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
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

pub(crate) type CustomResult<T> = Result<Json<T>, Custom<Json<Message>>>;

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
        Err(Custom(
            $status,
            Json(Message {
                message: $msg.to_string(),
            }),
        ))
    };
}
