use crate::error::ApiError;
use crate::route::{custom_message, SuccessMessage};
use crate::DOTENV_CONFIG;
use bson::oid::ObjectId;
use jsonwebtoken::{
  decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use k8s_openapi::chrono;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct Token {
  pub(crate) id: String,
  pub(crate) exp: i64,
  pub(crate) otp: Option<bool>,
}

impl Token {
  pub(crate) fn new(
    id: ObjectId,
    exp: i64,
    otp: Option<bool>,
  ) -> Self {
    Self {
      id: id.to_string(),
      exp,
      otp,
    }
  }

  pub(crate) fn new_with_duration(
    id: ObjectId,
    duration: &chrono::Duration,
    otp: Option<bool>,
  ) -> Result<Self, ApiError> {
    let expiration = chrono::Utc::now().checked_add_signed(duration.clone());
    match expiration {
      Some(exp) => Ok(Self::new(id, exp.timestamp(), otp)),
      None => Err(ApiError::new(
        Status::InternalServerError,
        "Error while generating token".to_string(),
      )),
    }
  }

  pub(crate) fn parse_authorization_header(
    header: &String
  ) -> Result<Token, ApiError> {
    if !header.starts_with("Bearer ") {
      return Err(ApiError::new(
        Status::Unauthorized,
        "Invalid authorization header".to_string(),
      ));
    }
    let token = &header[7..]; // Remove "Bearer " prefix
    Token::parse_jwt(&token.to_string())
  }

  pub(crate) fn parse_jwt(token: &String) -> Result<Token, ApiError> {
    let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
    let token_data = decode_token(token, &jwt_secret);

    return match token_data {
      Ok(token_data) => Ok(token_data.claims),
      Err(_) => {
        let message = "Error while parsing jwt token".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  pub(crate) fn parse_id(&self) -> Result<ObjectId, ApiError> {
    let result = ObjectId::from_str(self.id.as_str());
    return match result {
      Ok(id) => Ok(id),
      Err(_) => {
        let message = "Error while parsing token id".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  pub(crate) fn to_jwt(&self) -> Result<String, ApiError> {
    let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
    let jwt_encode = encode(&Header::default(), &self, &encoding_key);
    return match jwt_encode {
      Ok(token) => Ok(token),
      Err(_) => {
        let message = "Error while encoding jwt token".to_string();
        Err(ApiError::new(Status::InternalServerError, message))
      }
    };
  }

  pub(crate) fn is_expired(&self) -> bool {
    let now = chrono::Utc::now().timestamp();
    return self.exp < now;
  }

  pub(crate) fn with_otp(
    &mut self,
    otp: Option<bool>,
  ) {
    self.otp = otp;
  }

  pub(crate) fn with_id(
    &mut self,
    id: ObjectId,
  ) {
    self.id = id.to_string();
  }

  pub(crate) fn with_exp(
    &mut self,
    exp: i64,
  ) {
    self.exp = exp;
  }
}

pub(crate) fn gen_new_token(
  id: ObjectId,
  duration: &chrono::Duration,
  jwt_secret: &String,
  otp: Option<bool>,
) -> Result<String, jsonwebtoken::errors::Error> {
  let expiration = chrono::Utc::now()
    .checked_add_signed(duration.clone())
    .expect("valid timestamp")
    .timestamp();

  let claims = Token::new(id, expiration, otp);

  let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
  encode(&Header::default(), &claims, &encoding_key)
}

pub(crate) fn decode_token(
  token: &String,
  jwt_secret: &String,
) -> jsonwebtoken::errors::Result<TokenData<Token>> {
  let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
  decode::<Token>(token, &decoding_key, &Validation::default())
}

#[cfg(test)]
mod tests {
  use crate::guard::token::{decode_token, gen_new_token};
  use crate::DOTENV_CONFIG;
  use bson::oid::ObjectId;
  use k8s_openapi::chrono;

  #[test]
  fn test_gen_new_token() {
    let duration = chrono::Duration::hours(24);
    let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
    let new_token =
      gen_new_token(ObjectId::new(), &duration, &jwt_secret, None)
        .expect("Error generating token");
    assert!(new_token.len() > 0);
  }

  #[test]
  fn test_decode_token() {
    let duration = chrono::Duration::hours(24);
    let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
    let new_token =
      gen_new_token(ObjectId::new(), &duration, &jwt_secret, None)
        .expect("Error generating token");
    let token = decode_token(&new_token, &jwt_secret);
    assert!(token.is_ok());
    let token = token.unwrap();
    let token = token.claims;
    assert_eq!(token.id.len(), 24);
    assert!(token.exp > 0);
  }
}
