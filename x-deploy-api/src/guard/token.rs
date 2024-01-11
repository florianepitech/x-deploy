use crate::error::ApiError;
use crate::CONFIG;
use bson::oid::ObjectId;
use errors::ErrorKind;
use jsonwebtoken::{
  decode, encode, errors, DecodingKey, EncodingKey, Header, Validation,
};
use rocket::http::Status;
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
    otp: Option<bool>,
  ) -> Result<Self, ApiError> {
    let duration_min = CONFIG.jwt_key_duration_in_minutes;
    let duration = chrono::Duration::minutes(duration_min as i64);
    let expiration = chrono::Utc::now().checked_add_signed(duration.clone());
    match expiration {
      Some(exp) => Ok(Self {
        id: id.to_string(),
        exp: exp.timestamp(),
        otp,
      }),
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
    let jwt_secret = CONFIG.jwt_secret.clone();
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    let token_data =
      decode::<Token>(token, &decoding_key, &Validation::default());

    return match token_data {
      Ok(token_data) => Ok(token_data.claims),
      Err(e) => {
        let kind = e.kind();
        return match kind {
          ErrorKind::ExpiredSignature => {
            let message = "Token is expired, please login again".to_string();
            Err(ApiError::new(Status::Unauthorized, message))
          }
          _ => {
            let message = "Error while parsing jwt token".to_string();
            Err(ApiError::new(Status::InternalServerError, message))
          }
        };
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
    let jwt_secret = CONFIG.jwt_secret.clone();
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
