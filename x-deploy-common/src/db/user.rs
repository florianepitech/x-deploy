use crate::db::{CommonCollection, ToCollectionName};
use crate::CommonResult;
use bson::{doc, Bson};
use mongodb::bson::oid::ObjectId;
use mongodb::results::UpdateResult;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

const USER_COLLECTION_NAME: &str = "users";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[serde(rename = "firstname")]
  pub firstname: String,

  #[serde(rename = "lastname")]
  pub lastname: String,

  #[serde(rename = "profilePictureUrl")]
  pub profile_picture_url: Option<String>,

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
  pub fn is_enabled(&self) -> bool {
    return match self.setup {
      Some(_) => true,
      None => false,
    };
  }
}

impl User {
  pub fn new(
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
      profile_picture_url: None,
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

impl ToCollectionName for User {
  fn collection_name() -> String {
    return USER_COLLECTION_NAME.to_string();
  }
}

impl CommonCollection<User> {
  pub async fn find_with_email(
    &self,
    email: &String,
  ) -> CommonResult<Option<User>> {
    let user = self
      .collection
      .find_one(
        doc! {
          "email.email": email
        },
        None,
      )
      .await?;
    return Ok(user);
  }

  pub async fn email_confirm(
    &self,
    id: &ObjectId,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id
    };
    let update = doc! {
      "$set": {
        "email.verified": true,
        "email.code": Bson::Null
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }

  pub async fn password_update_hash(
    &self,
    id: &ObjectId,
    hash: &str,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id
    };
    let update = doc! {
      "$set": {
        "password": hash
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }

  pub async fn password_update_forgot_token(
    &self,
    id: &ObjectId,
    token: Option<&str>,
  ) -> CommonResult<UpdateResult> {
    let new_token = match token {
      None => Bson::Null,
      Some(token) => Bson::String(token.to_string()),
    };
    let filter = doc! {
      "_id": id
    };
    let update = doc! {
      "$set": {
        "password.tokenReset": new_token
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }

  pub async fn find_from_password_forgot_token(
    &self,
    token_reset: &str,
  ) -> CommonResult<Option<User>> {
    let filter = doc! {
      "password.tokenReset": token_reset
    };
    let result = self.collection.find_one(filter, None).await?;
    return Ok(result);
  }

  pub async fn change_phone(
    &self,
    id: &ObjectId,
    new_phone: &String,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id
    };
    let update = doc! {
      "$set": {
        "phone.phone": new_phone
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }

  pub async fn update_profile_picture_url(
    &self,
    id: &ObjectId,
    url: &String,
  ) -> CommonResult<UpdateResult> {
    let filter = doc! {
      "_id": id
    };
    let update = doc! {
      "$set": {
        "profilePictureUrl": url
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }

  pub async fn two_factor_update(
    &self,
    id: &ObjectId,
    two_factor: &Option<TwoFactor>,
  ) -> CommonResult<UpdateResult> {
    let new_value = match two_factor {
      Some(two_factor) => bson::to_bson(two_factor)?,
      None => Bson::Null,
    };
    let filter = doc! {
      "_id": id
    };
    let update = doc! {
      "$set": {
        "twoFactor": new_value
      }
    };
    let result = self.collection.update_one(filter, update, None).await?;
    return Ok(result);
  }
}
