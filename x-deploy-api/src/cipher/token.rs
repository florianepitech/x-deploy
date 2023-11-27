use bson::oid::ObjectId;
use jsonwebtoken::{DecodingKey, encode, decode, EncodingKey, Header, Validation, TokenData};
use k8s_openapi::chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Token {
    pub(crate) id: ObjectId,
    pub(crate) exp: String,
}

pub(crate) fn gen_new_token(
    id: ObjectId,
    duration: &chrono::Duration,
    jwt_secret: &String
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(duration.clone())
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Token {
        id,
        exp: expiration.to_string(),
    };

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