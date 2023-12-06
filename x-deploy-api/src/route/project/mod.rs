use bson::{doc, oid};
use crate::route::auth::dto::LoginResponse;
use crate::route::project::dto::CreateProjectBody;
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
use crate::db::project::Project;

mod dto;

macro_rules! get_organization_by_id {
    ($db:expr, $id:expr) => {
        async {
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

#[openapi(tag = "Project")]
#[post("/project", format = "application/json", data = "<body>")]
pub(crate) async fn new(
    db: &State<Database>,
    token: Token,
    body: Json<CreateProjectBody>,
) -> Result<Json<Message>, Custom<Json<Message>>> {
    // Verify if the id of the organization is valid
    // let organization_id = body.organization_id.clone();
    // let organization = get_organization_by_id!(db, organization_id);

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

#[openapi(tag = "Project")]
#[get("/project?<query..>", format = "application/json")]
pub(crate) async fn get_by_id(
    db: &State<Database>,
    query: dto::GetByIdQuery,
) -> Result<Json<LoginResponse>, Custom<Json<Message>>> {
    return custom_response!(Status::NotImplemented, "Not implemented");
}