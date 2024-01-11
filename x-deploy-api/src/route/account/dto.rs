use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "firstname": "John",
    "lastname": "Doe",
    "profilePictureUrl": "https://s3-custom-bucket.com/user/1234567890/profile_picture.png",
    "email": "john@doe.net",
    "emailVerified": true,
    "phone": "+34612345678",
}))]
pub(crate) struct GetAccountInfoResponse {
  #[serde(rename = "firstname")]
  pub(crate) firstname: String,

  #[serde(rename = "lastname")]
  pub(crate) lastname: String,

  #[serde(rename = "profilePictureUrl")]
  pub(crate) profile_picture_url: Option<String>,

  #[serde(rename = "email")]
  pub(crate) email: String,

  #[serde(rename = "emailVerified")]
  pub(crate) email_verified: bool,

  #[serde(rename = "phone")]
  pub(crate) phone: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "actualPassword": "MyActualStrongPassword123!",
    "newPassword": "MyNewAmazingStrongPassword123!",
}))]
pub(crate) struct ChangePasswordRequest {
  #[serde(rename = "actualPassword")]
  pub(crate) actual_password: String,

  #[serde(rename = "newPassword")]
  pub(crate) new_password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
  "code": "768905"
}))]
pub(crate) struct VerifyEmailRequest {
  #[serde(rename = "token")]
  pub(crate) code: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "newPhone": "+34612345678",
}))]
pub(crate) struct ChangePhoneRequest {
  #[serde(rename = "newPhone")]
  pub(crate) new_phone: String,
}

// =======================
// 2FA
// =======================

// => Setup

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "password": "MyActualStrongPassword123!",
}))]
pub(crate) struct TwoFactorSetupRequest {
  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "secret": "BA766BJBGJU...",
    "qrCode": "Ipofsjbkl9875UJIKkfds..."
}))]
pub(crate) struct TwoFactorSetupResponse {
  #[serde(rename = "recoveryCode")]
  pub(crate) recovery_code: String,

  #[serde(rename = "qrCode")]
  pub(crate) qr_code: String,
}

// => Info

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "password": "MyActualStrongPassword123!",
}))]
pub(crate) struct TwoFactorInfoRequest {
  #[serde(rename = "password")]
  pub(crate) password: String,
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "enabled": true,
    "secret": "BA766BJBGJU...",
    "qrCode": "Ipofsjbkl9875UJIKkfds...",
}))]
pub(crate) struct TwoFactorInfoResponse {
  #[serde(rename = "enabled")]
  pub(crate) enabled: bool,

  #[serde(rename = "secret")]
  pub(crate) secret: String,

  #[serde(rename = "qrCode")]
  pub(crate) qr_code: String,
}

// => Code

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[schema(example = json!({
    "code": "123456",
}))]
pub(crate) struct TwoFactorCodeRequest {
  #[serde(rename = "code")]
  pub(crate) code: String,
}
