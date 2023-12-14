use bson::oid;
use crate::route::{Message, MessageResult};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use crate::cipher::token::Token;
use crate::{custom_response, get_organization_by_id};
use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};

#[post("/organization/<id>/credentials/azure", format = "application/json")]
pub(crate) async fn new(
    db: &State<Database>,
    token: Token,
    id: String,
) -> MessageResult {
    let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[get("/organization/<id>/credentials/azure", format = "application/json")]
pub(crate) async fn get(
    db: &State<Database>,
    token: Token,
    id: String,
) -> MessageResult {
    let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[delete("/organization/<id>/credentials/azure", format = "application/json")]
pub(crate) async fn delete(
    db: &State<Database>,
    token: Token,
    id: String,
) -> MessageResult {
    let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}