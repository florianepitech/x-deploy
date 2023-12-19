use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::project::Project;
use crate::guard::token::Token;
use crate::route::organization::project::dto::CreateProjectBody;
use crate::route::{CustomResponse, Message};
use bson::{doc, oid};
use mongodb::{Collection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
mod dto;

/*
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
*/

#[post(
  "/organization/{id}/project",
  format = "application/json",
  data = "<body>"
)]
pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateProjectBody>,
) -> CustomResponse<Message> {
  controller::new(db, token, body).await
}

#[get("/organization/<id>/project", format = "application/json")]
pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: &str,
) -> CustomResponse<Message> {
  controller::get_by_id(db, token, id).await
}

#[patch("/organization/<id>/project/<project_id>", format = "application/json")]
pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: &str,
  project_id: &str,
) -> CustomResponse<Message> {
  controller::update(db, token, id, project_id).await
}

#[delete(
  "/organization/<id>/project/<project_id>",
  format = "application/json"
)]
pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: &str,
  project_id: &str,
) -> CustomResponse<Message> {
  controller::delete(db, token, id, project_id).await
}
