mod dto;

use std::str::FromStr;
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use ovh_api::OvhClient;
use crate::cipher::token::Token;
use crate::db::ovh_credentials::{OVH_CRED_COLLECTION_NAME, OvhCredentials};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::ovh::auth::test_ovh_connection;
use crate::route::Message;

#[post("/ovh/credentials", format = "application/json", data = "<body>")]
pub async fn post_credentials(db: &State<Database>, token: Token, body: Json<dto::Auth>) -> Result<Json<Message>, Custom<Json<Message>>> {
    let auth_body = body.into_inner();
    let client = OvhClient::new(
        auth_body.application_key,
        auth_body.application_secret,
        auth_body.consumer_key,
    );
    if (!test_ovh_connection(&client).await) {
        return Err(Custom(
            Status::Forbidden,
            Json(Message {
                message: "Credentials are incorrect".to_string(),
            }),
        ));
    }

    let mongodb_client = db.inner();
    let collection: Collection<OvhCredentials> = mongodb_client.collection(OVH_CRED_COLLECTION_NAME);
    let object_id = ObjectId::from_str(token.id.as_str());
    if object_id.is_err() {
        return Err(Custom(
            Status::BadRequest,
            Json(Message {
                message: "Malformed objectId in your token.".to_string(),
            }),
        ));
    }
    let user_id = object_id.unwrap();
    let ovh_credentials = OvhCredentials::new(
        client.application_key,
        client.application_secret,
        client.consumer_key,
        user_id,
    );
    collection.insert_one(ovh_credentials, None).await.unwrap();


    Ok(Json(Message {
        message: "Credentials added".to_string(),
    }))
}