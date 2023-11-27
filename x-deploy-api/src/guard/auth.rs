use rocket::{Request, request, outcome::Outcome};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::cipher::token::{decode_token, Token};
use crate::DOTENV_CONFIG;

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();

        if keys.len() != 1 {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }

        let key = keys[0];
        if !key.starts_with("Bearer ") {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }

        let token = &key[7..]; // Remove "Bearer " prefix
        let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
        let token = decode_token(&token.to_string(), &jwt_secret);
        if token.is_err() {
            return Outcome::Failure((rocket::http::Status::Unauthorized, ()));
        }
        let token = token.unwrap();
        let token = token.claims;
        return Outcome::Success(token);
    }
}
