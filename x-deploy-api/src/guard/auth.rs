use crate::guard::api_key::ApiKey;
use crate::guard::bearer_token::BearerToken;
use crate::route::ErrorMessage;
use request::FromRequest;
use rocket::{outcome::Outcome, request, Request};

pub enum Auth {
  Bearer(BearerToken),
  ApiKey(ApiKey),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
  type Error = ErrorMessage;

  async fn from_request(
    request: &'r Request<'_>
  ) -> request::Outcome<Self, Self::Error> {
    let bearer_token: request::Outcome<BearerToken, ErrorMessage> =
      BearerToken::from_request(request).await;
    if let Outcome::Success(token) = bearer_token {
      return Outcome::Success(Auth::Bearer(token));
    }
    let api_key: request::Outcome<ApiKey, ErrorMessage> =
      ApiKey::from_request(request).await;
    if let Outcome::Success(api_key) = api_key {
      return Outcome::Success(Auth::ApiKey(api_key));
    }
    let message = ErrorMessage::new(
      "Please provide a valid Bearer Token or Api Key to access this route"
        .to_string(),
    );
    return Outcome::Error((rocket::http::Status::Unauthorized, message));
  }
}
