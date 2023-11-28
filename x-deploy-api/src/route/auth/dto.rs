use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct LoginBody {
    #[serde(rename = "email")]
    pub(crate) email: String,

    #[serde(rename = "password")]
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct LoginResponse {
    #[serde(rename = "token")]
    pub(crate) token: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct RegisterBody {
    #[serde(rename = "firstname")]
    pub(crate) firstname: String,

    #[serde(rename = "lastname")]
    pub(crate) lastname: String,

    #[serde(rename = "email")]
    pub(crate) email: String,

    #[serde(rename = "phone")]
    pub(crate) phone: String,

    #[serde(rename = "password")]
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct AccountInfo {
    #[serde(rename = "firstname")]
    pub(crate) firstname: String,

    #[serde(rename = "lastname")]
    pub(crate) lastname: String,

    #[serde(rename = "email")]
    pub(crate) email: String,

    #[serde(rename = "phone")]
    pub(crate) phone: String,
}
