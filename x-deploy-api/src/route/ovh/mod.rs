mod dto;

use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use crate::db::ovh_credentials::{OvhCredentials, USER_COLLECTION_NAME};
use crate::ovh::auth::test_ovh_connection;
use crate::route::Message;

#[post("/ovh/credentials", format = "application/json", data = "<body>")]
pub async fn post_credentials( db: &State<Database>,body: Json<dto::Auth>) -> Result<Json<Message>, Custom<Json<Message>>> {
let auth_body = body.into_inner();
    let client = ovh_api::OvhClient::new(
        auth_body.application_key,
        auth_body.application_secret,
        auth_body.consumer_key,
    );
    if (!test_ovh_connection(client).await) {
        return Err(Custom(
            Status::Forbidden,
            Json(Message {
                message: "Credentials are incorrect".to_string(),
            }),
        ));
    }

    let mongodb_client = db.inner();
    let collection: Collection<OvhCredentials> = mongodb_client.collection(USER_COLLECTION_NAME

    Ok(Json(Message {
        message: "Credentials added".to_string(),
    }))
}