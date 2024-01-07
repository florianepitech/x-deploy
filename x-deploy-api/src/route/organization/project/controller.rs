use crate::guard::token::Token;
use crate::route::organization::project::dto::{
  CreateProjectRequest, ProjectInfoResponse, UpdateProjectInfoRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::ToObjectId;
use bson::oid;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::project::Project;
use x_deploy_common::db::query::project::{
  query_project_delete, query_project_get_with_org,
  query_project_get_with_org_and_id, query_project_new, query_project_update,
};

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  body: Json<CreateProjectRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;

  return match organization {
    Some(organization) => {
      let project = Project::new(
        body.name.clone(),
        body.description.clone(),
        organization.organization.id.clone(),
      );
      query_project_new(db.inner(), &project).await?;
      custom_message(Status::Created, "Your project was successfully created")
    }
    None => custom_message(
      Status::NotFound,
      "You are not a member of this organization",
    ),
  };
}

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResult<Vec<ProjectInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;

  return match organization {
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
    Some(organization) => {
      let projects =
        query_project_get_with_org(db.inner(), &organization.organization.id)
          .await?;
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
      custom_response(Status::Ok, result)
    }
  };
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
) -> ApiResult<ProjectInfoResponse> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let project_id = project_id.to_object_id()?;
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;

  return match organization {
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
    Some(organization) => {
      let project = query_project_get_with_org_and_id(
        db.inner(),
        &organization.organization.id,
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
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
  body: Json<UpdateProjectInfoRequest>,
) -> ApiResult<SuccessMessage> {
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
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;

  return match organization {
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
    Some(organization) => {
      let project = query_project_get_with_org_and_id(
        db.inner(),
        &organization.organization.id,
        &project_id,
      )
      .await?;
      return match project {
        Some(project) => {
          // TODO: Update project
          let result = query_project_update(
            db.inner(),
            &organization.organization.id,
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
  };
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  project_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let project_id = project_id.to_object_id()?;
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;

  return match organization {
    None => custom_error(
      Status::NotFound,
      "You are not a member of this organization",
    ),
    Some(organization) => {
      let project = query_project_get_with_org_and_id(
        db.inner(),
        &organization.organization.id,
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
  };
}
