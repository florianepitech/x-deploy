use crate::cipher::token::Token;
use crate::db::ovh_credentials::{OvhCredentials, OvhCredentialsStatus, OVH_CRED_COLLECTION_NAME};
use crate::db::user::{User, USER_COLLECTION_NAME};
use crate::ovh::auth::test_ovh_connection;
use crate::route::{Message, MessageResult};
use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Collection, Database};
use ovh_api::OvhClient;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use crate::custom_response;

#[post("/account/api-key", format = "application/json")]
pub(crate) async fn new(
    db: &State<Database>,
    token: Token,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[get("/account/api-key", format = "application/json")]
pub(crate) async fn get(
    db: &State<Database>,
    token: Token,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[get("/account/api-key/<id>", format = "application/json")]
pub(crate) async fn get_by_id(
    db: &State<Database>,
    token: Token,
    id: String,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[delete("/account/api-key/<id>", format = "application/json")]
pub(crate) async fn delete(
    db: &State<Database>,
    token: Token,
    id: String,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}