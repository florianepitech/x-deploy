use bson::{doc, oid};
use crate::route::{Message, MessageResult};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use crate::cipher::token::Token;
use crate::custom_response;
use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::project::Project;
use crate::route::organization::project::dto::CreateProjectBody;

mod dto;

#[macro_export]
macro_rules! get_organization_by_id {
    ($db:expr, $id:expr) => {
        async {
            use bson::doc;
            use mongodb::Collection;
            use rocket::http::Status;
            use crate::custom_response;
            use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
            let oid = match oid::ObjectId::parse_str(&$id) {
                Ok(oid) => oid,
                Err(_) => return custom_response!(Status::BadRequest, "Invalid organization id"),
            };

            let collection: Collection<Organization> = $db.collection(ORGANIZATION_COLLECTION_NAME);
            match collection.find_one(doc! {"_id": oid}, None).await {
                Ok(Some(organization)) => Ok(organization),
                Ok(None) => custom_response!(Status::NotFound, "Organization not found"),
                Err(_) => custom_response!(Status::InternalServerError, "An error occurred while getting your organization"),
            }
        }
    };
}

#[post("/organization/{id}/project", format = "application/json", data = "<body>")]
pub(crate) async fn new(
    db: &State<Database>,
    token: Token,
    body: Json<CreateProjectBody>,
) -> MessageResult {
    let organization = get_organization_by_id!(db, body.organization_id.clone()).await?;
    let project_new = Project::new(
        body.name.clone(),
        body.description.clone(),
        organization.id.clone(),
    );
    let collection: Collection<Project> = db.collection("projects");
    let result = collection.insert_one(project_new, None).await;
    if result.is_err() {
        return custom_response!(Status::InternalServerError, "An error occurred while creating your project");
    }
    let inserted_id = result.unwrap().inserted_id;
    info!("Inserted new project with id: {}", inserted_id);
    return custom_response!(Status::Created, "Your project has been created !");
}

#[get("/organization/<id>/project", format = "application/json")]
pub(crate) async fn get_by_id(
    db: &State<Database>,
    token: Token,
    id: &str,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[patch("/organization/<id>/project/<project_id>", format = "application/json")]
pub(crate) async fn update(
    db: &State<Database>,
    token: Token,
    id: &str,
    project_id: &str,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}

#[delete("/organization/<id>/project/<project_id>", format = "application/json")]
pub(crate) async fn delete(
    db: &State<Database>,
    token: Token,
    id: &str,
    project_id: &str,
) -> MessageResult {
    return custom_response!(Status::NotImplemented, "Not implemented");
}