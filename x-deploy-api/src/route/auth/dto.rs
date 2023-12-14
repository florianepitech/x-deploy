use rocket::serde::json::serde_json::json;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({"email": "john@doe.net", "password": "myAmazingStringPassword123!"}))]
pub(crate) struct LoginBody {
    #[serde(rename = "email")]
    pub(crate) email: String,

    #[serde(rename = "password")]
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub(crate) struct LoginResponse {
    #[serde(rename = "token")]
    pub(crate) token: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
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

#[derive(Deserialize, Serialize, Debug, ToSchema)]
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
