use crate::guard::token::Token;
use crate::route::ErrorMessage;
use request::FromRequest;
use rocket::{outcome::Outcome, request, Request};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
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
    let parse_header = Token::parse_authorization_header(&header.to_string());
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
