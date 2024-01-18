use crate::guard::auth::Auth;
use crate::permission::general::GeneralPermission;
use crate::route::organization::project::dto::{
  CreateProjectRequest, ProjectInfoResponse, UpdateProjectInfoRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::profile_picture::ProfilePicture;
use crate::CONFIG;
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::{Data, State};
use std::str::FromStr;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_project::OrganizationProject;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::CommonCollection;
use x_deploy_common::s3::bucket::CommonS3Bucket;
use x_deploy_common::s3::config::CommonS3Config;
use x_deploy_common::s3::file_type::CommonS3BucketType::ProjectLogo;

pub(crate) async fn new(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  body: Json<CreateProjectRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Project
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  // Create project
  let project = OrganizationProject::new(
    body.name.clone(),
    body.description.clone(),
    org_id,
  );
  let org_project_coll = CommonCollection::<OrganizationProject>::new(db);
  org_project_coll.insert_one(&project).await?;
  custom_message(Status::Created, "Your project was successfully created")
}

pub(crate) async fn get_all(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
) -> ApiResult<Vec<ProjectInfoResponse>> {
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Project
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  let org_project_coll = CommonCollection::<OrganizationProject>::new(db);
  let projects = org_project_coll.get_of_org(&org_id).await?;
  let mut result: Vec<ProjectInfoResponse> = Vec::new();
  for project in projects {
    let project_info = ProjectInfoResponse {
      id: project.id.to_hex(),
      name: project.name,
      description: project.description,
      logo_url: project.logo_url,
      organization_id: org_id.to_string(),
    };
    result.push(project_info);
  }
  custom_response(Status::Ok, result)
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
) -> ApiResult<ProjectInfoResponse> {
  let org_id = ObjectId::from_str(org_id)?;
  let project_id = ObjectId::from_str(project_id)?;

  GeneralPermission::Project
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;

  let org_project_coll = CommonCollection::<OrganizationProject>::new(db);
  let project = org_project_coll
    .get_with_id_of_org(&project_id, &org_id)
    .await?;
  return match project {
    Some(project) => {
      let result = ProjectInfoResponse {
        id: project.id.to_string(),
        name: project.name,
        description: project.description,
        logo_url: project.logo_url,
        organization_id: org_id.to_string(),
      };
      custom_response(Status::Ok, result)
    }
    None => return custom_error(Status::NotFound, "Project not found"),
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
  body: Json<UpdateProjectInfoRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(org_id)?;
  let project_id = ObjectId::from_str(project_id)?;

  GeneralPermission::Project
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  let org_project_coll = CommonCollection::<OrganizationProject>::new(db);
  let project = org_project_coll
    .get_with_id_of_org(&project_id, &org_id)
    .await?;
  return match project {
    Some(project) => {
      let result = org_project_coll
        .update_info(&project.id, &body.name, &body.description)
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
  auth: Auth,
  org_id: &str,
  project_id: &str,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(org_id)?;
  let project_id = ObjectId::from_str(project_id)?;

  GeneralPermission::Project
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  let org_project_coll = CommonCollection::<OrganizationProject>::new(db);
  let project = org_project_coll
    .get_with_id_of_org(&project_id, &org_id)
    .await?;
  return match project {
    Some(project) => {
      let result = org_project_coll.delete_by_id(&project.id).await?;
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

pub(crate) async fn update_logo(
  db: &State<Database>,
  auth: Auth,
  org_id: &str,
  project_id: &str,
  content_type: &ContentType,
  data: Data<'_>,
) -> ApiResult<SuccessMessage> {
  let org_id = ObjectId::from_str(org_id)?;
  let project_id = ObjectId::from_str(project_id)?;

  GeneralPermission::Project
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  let org_project_coll = CommonCollection::<OrganizationProject>::new(db);
  let project = match org_project_coll
    .get_with_id_of_org(&project_id, &org_id)
    .await?
  {
    Some(project) => project,
    None => return custom_error(Status::NotFound, "Project not found"),
  };
  let profile_picture = ProfilePicture::from_data(data).await?;
  let profile_picture = profile_picture.to_square()?;
  let s3_config = CommonS3Config::new(
    CONFIG.s3_endpoint.clone(),
    CONFIG.s3_bucket.clone(),
    CONFIG.s3_access_key.clone(),
    CONFIG.s3_secret_key.clone(),
    CONFIG.s3_region.clone(),
  );
  let extension = profile_picture.get_extension()?;
  let filename = format!("{}.{}", project_id, extension);
  let bytes = profile_picture.get_image_bytes()?;
  // Save file in S3
  let s3 = CommonS3Bucket::new(ProjectLogo, s3_config);
  let content_type_str = content_type.to_string();
  s3.add(&filename, bytes.as_slice(), content_type_str)
    .await?;
  // Update project logo url
  let url = s3.get_public_url(&filename);
  org_project_coll
    .update_logo_url(&project.id, &Some(url))
    .await?;
  custom_message(Status::Ok, "The logo of your project is now updated")
}
