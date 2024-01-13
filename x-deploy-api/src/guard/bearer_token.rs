use crate::error::ApiError;
use crate::route::ErrorMessage;
use crate::CONFIG;
use bson::oid::ObjectId;
use errors::ErrorKind;
use jsonwebtoken::{
  decode, encode, errors, DecodingKey, EncodingKey, Header, Validation,
};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct BearerToken {
  pub id: String,
  pub exp: i64,
  pub otp: Option<bool>,
}

impl BearerToken {
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
  ) -> Result<BearerToken, ApiError> {
    if !header.starts_with("Bearer ") {
      return Err(ApiError::new(
        Status::Unauthorized,
        "Invalid authorization header".to_string(),
      ));
    }
    let token = &header[7..]; // Remove "Bearer " prefix
    BearerToken::parse_jwt(&token.to_string())
  }

  pub(crate) fn parse_jwt(token: &String) -> Result<BearerToken, ApiError> {
    let jwt_secret = CONFIG.jwt_secret.clone();
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    let token_data =
      decode::<BearerToken>(token, &decoding_key, &Validation::default());

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
  type Error = ErrorMessage;

  async fn from_request(
    req: &'r Request<'_>
  ) -> request::Outcome<Self, Self::Error> {
    let keys: Vec<_> = req.headers().get("Authorization").collect();

    if keys.len() != 1 {
      let message =
        ErrorMessage::new("Authorization header must be present".to_string());
      return Outcome::Error((rocket::http::Status::Unauthorized, message));
    }

    let header = keys[0];
    let parse_header =
      BearerToken::parse_authorization_header(&header.to_string());
    return match parse_header {
      Ok(token) => {
        // Verify if token is expired
        if (token.is_expired()) {
          return Outcome::Error((
            rocket::http::Status::Unauthorized,
            ErrorMessage::new("Token expired, please login again.".to_string()),
          ));
        }
        // Verify if 2FA is validated
        if let Some(otp) = token.otp {
          if !otp {
            return Outcome::Error((
              rocket::http::Status::Unauthorized,
              ErrorMessage::new("2FA not validated".to_string()),
            ));
          }
        }
        Outcome::Success(token)
      }
      Err(e) => Outcome::Error((e.status, ErrorMessage::new(e.message))),
    };
  }
}
