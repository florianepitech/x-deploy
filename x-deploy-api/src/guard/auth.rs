use crate::guard::token::{decode_token, Token};
use crate::route::Message;
use crate::DOTENV_CONFIG;
use k8s_openapi::chrono;
use rocket::{outcome::Outcome, request, Request};

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for Token {
  type Error = Message;

  async fn from_request(
    req: &'r Request<'_>
  ) -> request::Outcome<Self, Self::Error> {
    let keys: Vec<_> = req.headers().get("Authorization").collect();

    if keys.len() != 1 {
      let message =
        Message::new("Authorization header must be present".to_string());
      return Outcome::Error((rocket::http::Status::Unauthorized, message));
    }

    let key = keys[0];
    if !key.starts_with("Bearer ") {
      let message =
        Message::new("Authorization header must start with Bearer".to_string());
      return Outcome::Error((rocket::http::Status::Unauthorized, message));
    }

    let token = &key[7..]; // Remove "Bearer " prefix
    let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
    let token = decode_token(&token.to_string(), &jwt_secret);
    if token.is_err() {
      let message = Message::new("Invalid token".to_string());
      return Outcome::Error((rocket::http::Status::Unauthorized, message));
    }
    let token = token.unwrap();
    let token = token.claims;
    // Verify if the token is expired
    let now = chrono::Utc::now().timestamp();
    if token.exp < now {
      let message =
        Message::new("Token expired, please login again.".to_string());
      return Outcome::Error((rocket::http::Status::Unauthorized, message));
    }
    return Outcome::Success(token);
  }
}
