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
    "registeredAt": "2024-01-01T00:00:00Z",
}))]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetAccountInfoResponse {
  pub(crate) firstname: String,
  pub(crate) lastname: String,
  pub(crate) profile_picture_url: Option<String>,
  pub(crate) email: String,
  pub(crate) email_verified: bool,
  pub(crate) phone: String,
  pub(crate) phone_verified: bool,
  pub(crate) registered_at: String,
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
