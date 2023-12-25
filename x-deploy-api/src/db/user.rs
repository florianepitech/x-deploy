use crate::error::ApiError;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub(crate) const USER_COLLECTION_NAME: &str = "users";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "firstname")]
  pub firstname: String,

  #[serde(rename = "lastname")]
  pub lastname: String,

  #[serde(rename = "password")]
  pub password: Password,

  #[serde(rename = "twoFactor")]
  pub two_factor: Option<TwoFactor>,

  #[serde(rename = "email")]
  pub email: Email,

  #[serde(rename = "phone")]
  pub phone: Phone,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Email {
  #[serde(rename = "email")]
  pub email: String,

  #[serde(rename = "verified")]
  pub verified: bool,

  #[serde(rename = "code")]
  pub code: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Phone {
  #[serde(rename = "phone")]
  pub phone: String,

  #[serde(rename = "verified")]
  pub verified: bool,

  #[serde(rename = "code")]
  pub code: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Password {
  #[serde(rename = "password")]
  pub password: String,

  #[serde(rename = "lastChanged")]
  pub last_changed: Option<chrono::DateTime<chrono::Utc>>,

  #[serde(rename = "tokenReset")]
  pub token_reset: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TwoFactor {
  #[serde(rename = "secretBase32")]
  pub secret_base32: String,

  #[serde(rename = "recoveryCode")]
  pub recovery_code: String,

  #[serde(rename = "setup")]
  pub setup: Option<bson::DateTime>,
}

impl TwoFactor {
  pub(crate) fn is_enabled(&self) -> bool {
    return match self.setup {
      Some(_) => true,
      None => false,
    };
  }
}

impl User {
  pub(crate) fn new(
    firstname: String,
    lastname: String,
    password: String,
    email: String,
    phone: String,
  ) -> Self {
    User {
      id: ObjectId::new(),
      firstname,
      lastname,
      password: Password {
        password,
        last_changed: None,
        token_reset: None,
      },
      two_factor: None,
      email: Email {
        email,
        verified: false,
        code: None,
      },
      phone: Phone {
        phone,
        verified: false,
        code: None,
      },
    }
  }

  pub(crate) fn verify_password(
    &self,
    password: &str,
  ) -> Result<bool, ApiError> {
    return crate::cipher::password::verify_password(
      password,
      &self.password.password,
    );
  }
}
