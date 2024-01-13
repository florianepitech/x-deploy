use crate::route::ErrorMessage;
use bson::oid::ObjectId;
use chrono::Utc;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::{request, Request};
use serde::{Deserialize, Serialize};
use x_deploy_common::db::organization_apikey::OrganizationApiKey;
use x_deploy_common::db::organization_role::OrganizationRole;
use x_deploy_common::db::query::organization_api_key::OrganizationApiKeyQuery;
use x_deploy_common::db::CommonCollection;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
  pub id: ObjectId,
  pub value: String,
  pub exp: Option<chrono::DateTime<Utc>>,
  pub org_id: ObjectId,
  pub role: Option<OrganizationRole>,
}

impl ApiKey {
  pub fn new(
    id: ObjectId,
    value: String,
    exp: Option<chrono::DateTime<Utc>>,
    org_id: ObjectId,
    role: Option<OrganizationRole>,
  ) -> Self {
    Self {
      id,
      value,
      exp,
      org_id,
      role,
    }
  }

  pub fn is_expired(&self) -> bool {
    match self.exp {
      Some(exp) => exp < Utc::now(),
      None => false,
    }
  }
}

impl From<OrganizationApiKeyQuery> for ApiKey {
  fn from(value: OrganizationApiKeyQuery) -> Self {
    let expires_at = match value.expires_at {
      Some(expires_at) => Some(expires_at.to_chrono()),
      None => None,
    };
    Self {
      id: value.id,
      value: value.key,
      exp: expires_at,
      org_id: value.organization_id,
      role: value.role,
    }
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
  type Error = ErrorMessage;

  async fn from_request(
    req: &'r Request<'_>
  ) -> request::Outcome<Self, Self::Error> {
    let api_key_value = match req.headers().get_one("Authorization") {
      Some(keys) => keys,
      None => {
        let message =
          ErrorMessage::new("Authorization header must be present".to_string());
        return Outcome::Error((Status::Unauthorized, message));
      }
    };

    let db = match req.rocket().state::<mongodb::Database>() {
      Some(db) => db,
      None => {
        let message = ErrorMessage::new(
          "Database connection error for validate your authentication"
            .to_string(),
        );
        return Outcome::Error((Status::InternalServerError, message));
      }
    };

    let akc = CommonCollection::<OrganizationApiKey>::new(db);
    let api_key = match akc.get_by_value(api_key_value).await {
      Ok(api_key) => api_key,
      Err(_) => {
        let message =
          ErrorMessage::new("Failed to verify api key in database".to_string());
        return Outcome::Error((Status::InternalServerError, message));
      }
    };

    let api_key = match api_key {
      Some(api_key) => api_key,
      None => {
        let message = ErrorMessage::new("Invalid api key".to_string());
        return Outcome::Error((Status::Unauthorized, message));
      }
    };
    let result: ApiKey = api_key.into();

    if result.is_expired() {
      let message = ErrorMessage::new(
        "Api key is expired, please login again.".to_string(),
      );
      return Outcome::Error((Status::Unauthorized, message));
    }

    return Outcome::Success(result);
  }
}
