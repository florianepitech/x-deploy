use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct GetAccountInfoResponse {
    #[serde(rename = "firstname")]
    pub(crate) firstname: String,

    #[serde(rename = "lastname")]
    pub(crate) name: String,

    #[serde(rename = "email")]
    pub(crate) email: String,

    #[serde(rename = "emailVerified")]
    pub(crate) email_verified: bool,

}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct ChangePasswordBody {
    #[serde(rename = "newPassword")]
    pub(crate) new_password: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct VerifyEmailBody {
    #[serde(rename = "token")]
    pub(crate) token: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub(crate) struct ChangePhoneBody {
    #[serde(rename = "newPhone")]
    pub(crate) new_phone: String,
}
