use bson::Bson::ObjectId;
use bson::oid;
use crate::route::organization::dto::{CreateOrganizationBody, GetByIdQuery};
use crate::route::Message;
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::cipher::token::Token;
use crate::custom_response;
use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::route::auth::info;

pub(crate) mod dto;

#[openapi(tag = "Organization")]
#[post("/organization", format = "application/json", data = "<body>")]
pub(crate) async fn new(
    db: &State<Database>,
    token: Token,
    body: Json<CreateOrganizationBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    let collection: Collection<Organization> = db.collection(ORGANIZATION_COLLECTION_NAME);
    // Get objectId from token
    let owner = oid::ObjectId::parse_str(&token.id);
    if owner.is_err() {
        return custom_response!(Status::BadRequest, "Invalid token");
    }
    let new_organization = Organization::new(
        body.name.clone(),
        body.description.clone(),
        body.website.clone(),
        body.contact_email.clone(),
        owner.unwrap(),
    );
    let result = collection.insert_one(new_organization, None).await;
    if (result.is_err()) {
        return custom_response!(Status::InternalServerError, "An error occurred while creating your organization");
    }
    let inserted_id = result.unwrap().inserted_id;
    info!("Inserted new organization with id: {}", inserted_id);
    Ok(Json(Message {
        message: format!("Your organization has been created !")
    }))
}

#[openapi(tag = "Organization")]
#[get("/organization?<query..>", format = "application/json")]
pub(crate) async fn get_by_id(
    db: &State<Database>,
    query: GetByIdQuery,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    Err(Custom(
        Status::NotImplemented,
        Json(Message {
            message: format!("Not implemented: {}", query.id),
        }),
    ))
}
