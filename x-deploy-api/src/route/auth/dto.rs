use rocket::serde::json::serde_json::json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "email": "john@doe.net",
    "password": "myAmazingStringPassword123!"
}))]
pub(crate) struct LoginBody {
  #[serde(rename = "email")]
  pub(crate) email: String,

  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "email": "john@doe.net"
}))]
pub(crate) struct MagicLinkBody {
  pub(crate) email: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "token": "ey6b0pm7hk87bJB...",
    "code": "123678"
}))]
pub(crate) struct TwoFactorCode {
  #[serde(rename = "token")]
  pub(crate) token: String,

  #[serde(rename = "code")]
  pub(crate) code: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "token": "ey6b0pm7hk87bJB...",
    "recoveryCode": "123678JFDF86FDSF786Y..."
}))]
pub(crate) struct TwoFactorRecoveryBody {
  #[serde(rename = "token")]
  pub(crate) token: String,

  #[serde(rename = "recoveryCode")]
  pub(crate) recovery_code: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "token": "ey6b0pm7hk87bJB..."
}))]
pub(crate) struct LoginResponse {
  #[serde(rename = "token")]
  pub(crate) token: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "firstname": "John",
    "lastname": "DOE",
    "email": "john@doe.net",
    "phone": "+1234567890",
    "password": "myAmazingStringPassword123!"
}))]
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
#[schema(example = json!({
    "email": "john@doe.net",
}))]
pub(crate) struct ForgotPasswordBody {
  #[serde(rename = "email")]
  pub(crate) email: String,
}
