use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginBody {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LoginResponse {
    pub(crate) token: String,
}

#[derive(Deserialize, Serialize, Debug)]
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