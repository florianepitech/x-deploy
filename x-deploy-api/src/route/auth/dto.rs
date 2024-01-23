use crate::oauth::OAuthService;
use rocket::serde::json::serde_json::json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "email": "john@doe.net",
    "password": "myAmazingStringPassword123!"
}))]
pub(crate) struct LoginRequest {
  #[validate(email(message = "Your email is in a wrong format"))]
  #[serde(rename = "email")]
  pub(crate) email: String,

  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
  "service": "GITHUB",
  "accessToken": "gh_ey78...",
}))]
pub(crate) struct LoginOAuthRequest {
  pub(crate) service: OAuthServiceType,
  pub(crate) access_token: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub enum OAuthServiceType {
  #[serde(rename = "GITHUB")]
  Github,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "email": "john@doe.net"
}))]
pub(crate) struct MagicLinkRequest {
  #[validate(email(message = "Your email is in a wrong format"))]
  #[serde(rename = "email")]
  pub(crate) email: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "token": "ey6b0pm7hk87bJB...",
    "code": "123678"
}))]
pub(crate) struct TwoFactorCodeRequest {
  #[serde(rename = "token")]
  pub(crate) token: String,

  #[serde(rename = "code")]
  pub(crate) code: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "token": "ey6b0pm7hk87bJB...",
    "recoveryCode": "123678JFDF86FDSF786Y..."
}))]
pub(crate) struct TwoFactorRecoveryRequest {
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

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "firstname": "John",
    "lastname": "DOE",
    "email": "john@doe.net",
    "phone": "+1234567890",
    "password": "myAmazingStringPassword123!"
}))]
pub(crate) struct RegisterRequest {
  #[validate(length(
    min = 2,
    max = 50,
    message = "Your firstname is too short"
  ))]
  #[serde(rename = "firstname")]
  pub(crate) firstname: String,

  #[validate(length(
    min = 2,
    max = 50,
    message = "Your lastname is too short"
  ))]
  #[serde(rename = "lastname")]
  pub(crate) lastname: String,

  #[validate(email(message = "Your email is in a wrong format"))]
  #[serde(rename = "email")]
  pub(crate) email: String,

  #[validate(phone(message = "Your phone is in a wrong format"))]
  #[serde(rename = "phone")]
  pub(crate) phone: String,

  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "email": "john@doe.net",
}))]
pub(crate) struct ForgotPasswordRequest {
  #[validate(email(message = "Your email is in a wrong format"))]
  #[serde(rename = "email")]
  pub(crate) email: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, Validate)]
#[schema(example = json!({
    "token": "povshdiobndsolnOIU98YY97FGDIshkbf...",
    "newPassword": "myAmazingStringPassword123!"
}))]
pub(crate) struct ResetPasswordRequest {
  #[serde(rename = "token")]
  pub(crate) token: String,

  #[serde(rename = "newPassword")]
  pub(crate) new_password: String,
}

impl Into<OAuthService> for OAuthServiceType {
  fn into(self) -> OAuthService {
    match self {
      OAuthServiceType::Github => OAuthService::Github,
    }
  }
}
