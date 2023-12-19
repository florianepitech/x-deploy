use k8s_openapi::chrono;
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
  #[serde(rename = "enabled")]
  pub enabled: bool,

  #[serde(rename = "secret")]
  pub secret: Vec<u8>,

  #[serde(rename = "description")]
  pub description: String,

  #[serde(rename = "created")]
  pub created: chrono::DateTime<chrono::Utc>,
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
}
