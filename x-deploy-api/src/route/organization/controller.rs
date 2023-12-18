use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::guard::token::Token;
use crate::route::organization::dto::CreateOrganizationBody;
use crate::route::organization::dto::TransferOrganizationBody;
use crate::route::{Message, MessageResult};
use crate::{custom_message, custom_response, DOTENV_CONFIG};
use bson::{doc, oid};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) async fn new(
    db: &State<Database>,
    token: Token,
    body: Json<CreateOrganizationBody>,
) -> MessageResult {
    let owner = oid::ObjectId::parse_str(&token.id).unwrap();
    let is_allowed = verify_number_of_created_organization(db, &owner).await;
    if is_allowed.is_some() {
        return Err(is_allowed.unwrap());
    }
    let collection: Collection<Organization> = db.collection(ORGANIZATION_COLLECTION_NAME);
    // Get objectId from token
    let new_organization = Organization::new(
        body.name.clone(),
        body.description.clone(),
        body.website.clone(),
        body.contact_email.clone(),
        owner,
    );
    let result = collection.insert_one(new_organization, None).await;
    if result.is_err() {
        return custom_response!(
            Status::InternalServerError,
            "An error occurred while creating your organization"
        );
    }
    let inserted_id = result.unwrap().inserted_id;
    info!("Inserted new organization with id: {}", inserted_id);
    custom_message!(Status::Ok, "Organization created successfully")
}

pub(crate) async fn get_by_id(db: &State<Database>, id: String) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn update(db: &State<Database>, token: Token, id: String) -> MessageResult {
    // let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(db: &State<Database>, token: Token, id: String) -> MessageResult {
    // let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn transfer(
    db: &State<Database>,
    token: Token,
    id: String,
    body: Json<TransferOrganizationBody>,
) -> MessageResult {
    // let organization = get_organization_by_id!(db, id).await?;
    return custom_response!(Status::NotImplemented, "Not implemented");
}

// Utils method

async fn verify_number_of_created_organization(
    db: &State<Database>,
    user_id: &oid::ObjectId,
) -> Option<Custom<Json<Message>>> {
    let max_by_owner = DOTENV_CONFIG.max_organization_by_owner;
    let collection: Collection<Organization> = db.collection(ORGANIZATION_COLLECTION_NAME);
    let count = collection
        .count_documents(
            doc! {
                "owner": user_id
            },
            None,
        )
        .await;
    if count.is_err() {
        return Some(Custom(
            Status::InternalServerError,
            Json(Message::new(
                "An error occurred while creating your organization".to_string(),
            )),
        ));
    }
    let count = count.unwrap();
    if count >= max_by_owner {
        let message = format!("You can't create more than {} organization", max_by_owner);
        return Some(Custom(Status::Forbidden, Json(Message::new(message))));
    }
    return None;
}
