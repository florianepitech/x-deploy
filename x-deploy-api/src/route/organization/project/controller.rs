use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::project::Project;
use crate::guard::token::Token;
use crate::route::organization::project::dto::CreateProjectBody;
use crate::route::{custom_message, CustomResponse, Message};
use bson::{doc, oid};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateProjectBody>,
) -> CustomResponse<Message> {
  // let organization = get_organization_by_id!(db, body.organization_id.clone()).await?;
  // let project_new = Project::new(
  //     body.name.clone(),
  //     body.description.clone(),
  //     organization.id.clone(),
  // );
  // let collection: Collection<Project> = db.collection("projects");
  // let result = collection.insert_one(project_new, None).await;
  // if result.is_err() {
  //     return custom_message(Status::InternalServerError, "An error occurred while creating your project");
  // }
  // let inserted_id = result.unwrap().inserted_id;
  // info!("Inserted new project with id: {}", inserted_id);
  // return custom_message(Status::Created, "Your project has been created !");
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: &str,
) -> CustomResponse<Message> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: &str,
  project_id: &str,
) -> CustomResponse<Message> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: &str,
  project_id: &str,
) -> CustomResponse<Message> {
  return custom_message(Status::NotImplemented, "Not implemented");
}
