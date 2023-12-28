use crate::db::project::Project;
use crate::db::query::organization::get_org_by_id_with_owner;
use crate::db::query::project::{
  query_project_delete, query_project_get_with_org,
  query_project_get_with_org_and_id, query_project_new, query_project_update,
};
use crate::guard::token::Token;
use crate::route::organization::project::dto::{
  CreateProjectBody, ProjectInfoResponse, UpdateProjectInfoBody,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResponse, SuccessMessage,
};
use bson::oid;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  body: Json<CreateProjectBody>,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::from_str(org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_message(
        Status::BadRequest,
        "Organization id is not a valid ObjectId",
      )
    }
  };
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let project = Project::new(
    body.name.clone(),
    body.description.clone(),
    organization.id.clone(),
  );
  query_project_new(db.inner(), &project).await?;
  return custom_message(
    Status::Created,
    "Your project was successfully created",
  );
}

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResponse<Vec<ProjectInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::from_str(org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Organization id is not a valid ObjectId",
      )
    }
  };
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let projects =
    query_project_get_with_org(db.inner(), &organization.id).await?;

  let mut result: Vec<ProjectInfoResponse> = Vec::new();
  for project in projects {
    let project_info = ProjectInfoResponse {
      id: project.id.to_hex(),
      name: project.name,
      description: project.description,
      organization_id: org_id.to_string(),
    };
    result.push(project_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
) -> ApiResponse<ProjectInfoResponse> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::from_str(org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Organization id is not a valid id",
      )
    }
  };
  let project_id = match oid::ObjectId::from_str(project_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Project id is not a valid id")
    }
  };
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let project = query_project_get_with_org_and_id(
    db.inner(),
    &organization.id,
    &project_id,
  )
  .await?;
  return match project {
    Some(project) => {
      let result = ProjectInfoResponse {
        id: project.id.to_string(),
        name: project.name,
        description: project.description,
        organization_id: org_id.to_string(),
      };
      custom_response(Status::Ok, result)
    }
    None => return custom_error(Status::NotFound, "Project not found"),
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
  body: Json<UpdateProjectInfoBody>,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::from_str(org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Organization id is not a valid id",
      )
    }
  };
  let project_id = match oid::ObjectId::from_str(project_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Project id is not a valid id")
    }
  };
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let project = query_project_get_with_org_and_id(
    db.inner(),
    &organization.id,
    &project_id,
  )
  .await?;
  return match project {
    Some(project) => {
      // TODO: Update project
      let result = query_project_update(
        db.inner(),
        &organization.id,
        &project.id,
        &body.name,
        &body.description,
      )
      .await?;
      if result.modified_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Project not updated",
        );
      }
      custom_message(Status::Ok, "Project updated")
    }
    None => return custom_error(Status::NotFound, "Project not found"),
  };
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::from_str(org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Organization id is not a valid id",
      )
    }
  };
  let project_id = match oid::ObjectId::from_str(project_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Project id is not a valid id")
    }
  };
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let project = query_project_get_with_org_and_id(
    db.inner(),
    &organization.id,
    &project_id,
  )
  .await?;
  return match project {
    Some(project) => {
      let result =
        query_project_delete(db.inner(), &org_id, &project.id).await?;
      if result.deleted_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Project not deleted",
        );
      }
      custom_message(Status::Ok, "Project deleted")
    }
    None => return custom_error(Status::NotFound, "Project not found"),
  };
}
