use bson::oid::ObjectId;
use jsonwebtoken::{DecodingKey, encode, decode, EncodingKey, Header, Validation, TokenData};
use k8s_openapi::chrono;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::OpenApiFromRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, OpenApiFromRequest)]
pub(crate) struct Token {
    pub(crate) id: String,
    pub(crate) exp: i64,
}

impl Token {
    pub(crate) fn new(id: ObjectId, exp: i64) -> Self {
        Self {
            id: id.to_string(),
            exp,
        }
    }
}

pub(crate) fn gen_new_token(
    id: ObjectId,
    duration: &chrono::Duration,
    jwt_secret: &String
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(duration.clone())
        .expect("valid timestamp")
        .timestamp();

    let claims = Token::new(id, expiration);

    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
    encode(&Header::default(), &claims, &encoding_key)
}

pub(crate) fn decode_token(
    token: &String,
    jwt_secret: &String
) -> jsonwebtoken::errors::Result<TokenData<Token>> {
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    decode::<Token>(token, &decoding_key, &Validation::default())
}

#[cfg(test)]
mod tests {
    use crate::cipher::token::{gen_new_token, decode_token};
    use crate::DOTENV_CONFIG;
    use bson::oid::ObjectId;
    use k8s_openapi::chrono;

    #[test]
    fn test_gen_new_token() {
        let duration = chrono::Duration::hours(24);
        let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
        let new_token = gen_new_token(
            ObjectId::new(),
            &duration,
            &jwt_secret,
        ).expect("Error generating token");
        assert!(new_token.len() > 0);
    }

    #[test]
    fn test_decode_token() {
        let duration = chrono::Duration::hours(24);
        let jwt_secret = DOTENV_CONFIG.jwt_secret.clone();
        let new_token = gen_new_token(
            ObjectId::new(),
            &duration,
            &jwt_secret,
        ).expect("Error generating token");
        let token = decode_token(&new_token, &jwt_secret);
        assert!(token.is_ok());
        let token = token.unwrap();
        let token = token.claims;
        assert_eq!(token.id.len(), 24);
        assert!(token.exp > 0);
    }
}