use crate::guard::auth::Auth;
use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::dto::{
  CreateOrganizationRequest, OrganizationInfoResponse,
  UpdateOrganizationRequest,
};
use crate::route::organization::dto::{
  DeleteOrganizationRequest, TransferOrganizationRequest,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::password::verify_password;
use crate::utils::profile_picture::ProfilePicture;
use crate::CONFIG;
use bson::oid;
use bson::Bson::ObjectId;
use mongodb::Database;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::{Data, State};
use std::str::FromStr;
use x_deploy_common::db::organization::Organization;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::StandardPermission;
use x_deploy_common::db::user::User;
use x_deploy_common::db::CommonCollection;
use x_deploy_common::event::organization::OrganizationCreatedEvent;
use x_deploy_common::event::CommonEvent;
use x_deploy_common::s3::bucket::CommonS3Bucket;
use x_deploy_common::s3::config::CommonS3Config;
use x_deploy_common::s3::file_type::CommonS3BucketType::OrganizationLogo;

pub(crate) async fn all(
  db: &State<Database>,
  token: BearerToken,
) -> ApiResult<Vec<OrganizationInfoResponse>> {
  let id = token.parse_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let orgs = org_member_coll.get_all_org_of_user(&id).await?;
  let mut result: Vec<OrganizationInfoResponse> = Vec::new();
  for org in orgs {
    let org_info = OrganizationInfoResponse {
      id: org.organization.id.to_string(),
      name: org.organization.name,
      description: org.organization.description,
      logo_url: org.organization.logo_url,
      website: org.organization.website,
      contact_email: org.organization.contact_email,
    };
    result.push(org_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn new(
  db: &State<Database>,
  token: BearerToken,
  body: Json<CreateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  // Insert Organization in database
  let org_collection = CommonCollection::<Organization>::new(db);
  let new_organization = Organization::new(
    body.name.clone(),
    body.description.clone(),
    body.website.clone(),
    body.contact_email.clone(),
  );
  org_collection.insert_one(&new_organization).await?;
  let inserted_id = new_organization.id.clone();
  // Insert Organization member as owner
  let org_member_collection = CommonCollection::<OrganizationMember>::new(db);
  let owner = OrganizationMember::new(inserted_id.clone(), user_id, true, None);
  org_member_collection.insert_one(&owner).await?;

  CommonEvent::new(CONFIG.kafka_url.clone()).send(
    OrganizationCreatedEvent {
      id: inserted_id.clone().to_string(),
      name: body.name.clone(),
      description: body.description.clone(),
      creator_id: user.id.to_string(),
      creator_firstname: user.firstname.clone(),
      creator_lastname: user.lastname.clone(),
      creator_email: user.email.email.clone(),
    },
  )?;

  info!("Inserted new organization with id: {}", inserted_id);
  custom_message(Status::Ok, "Organization created successfully")
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  auth: Auth,
  org_id: String,
) -> ApiResult<OrganizationInfoResponse> {
  let org_id = bson::oid::ObjectId::from_str(&org_id)?;
  GeneralPermission::Organization
    .verify_auth(db, auth, &org_id, StandardPermission::Read)
    .await?;
  let collection = CommonCollection::<Organization>::new(db);
  let org = collection.get_by_id(&org_id).await?;
  return match org {
    Some(org) => {
      let result = OrganizationInfoResponse {
        id: org.id.to_string(),
        name: org.name,
        description: org.description,
        logo_url: org.logo_url,
        website: org.website,
        contact_email: org.contact_email,
      };
      custom_response(Status::Ok, result)
    }
    None => custom_error(
      Status::InternalServerError,
      "Organization not found, please contact support",
    ),
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  auth: Auth,
  org_id: String,
  body: Json<UpdateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  let org_id = bson::oid::ObjectId::from_str(&org_id)?;

  GeneralPermission::Organization
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

  let org_collection = CommonCollection::<Organization>::new(db);
  let update_result = org_collection
    .update_info(
      &org_id,
      body.name.clone(),
      body.description.clone(),
      body.website.clone(),
      body.contact_email.clone(),
    )
    .await?;
  if update_result.modified_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Failed to update organization",
    );
  }
  custom_message(Status::Ok, "Organization updated successfully")
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: BearerToken,
  id: String,
  body: Json<DeleteOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let password = body.password.clone();
  let user_collection = CommonCollection::<User>::new(db);
  let user = match user_collection.get_by_id(&user_id).await? {
    Some(user) => user,
    None => return custom_error(Status::NotFound, "User not found"),
  };
  let verify_password =
    verify_password(password.as_str(), user.password.password.as_str())?;
  if !verify_password {
    return custom_error(
      Status::Forbidden,
      "Invalid password for delete organization",
    );
  }
  let org_id = match oid::ObjectId::parse_str(&id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Invalid organization id")
    }
  };
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let organization = org_member_coll.get_user_in_org(&org_id, &user_id).await?;

  return match organization {
    None => custom_error(Status::NotFound, "Organization not found"),
    Some(organization) => {
      let org_collection = CommonCollection::<Organization>::new(db);
      let result = org_collection.delete_by_id(&organization.id).await?;
      if result.deleted_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Failed to delete organization",
        );
      }
      // TODO: Delete member, custom role... etc
      return custom_message(Status::Ok, "Organization deleted successfully");
    }
  };
}

pub(crate) async fn transfer(
  db: &State<Database>,
  token: BearerToken,
  id: String,
  body: Json<TransferOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  return custom_message(Status::NotImplemented, "Not implemented");
}

pub(crate) async fn update_logo(
  db: &State<Database>,
  auth: Auth,
  org_id: String,
  content_type: &ContentType,
  data: Data<'_>,
) -> ApiResult<SuccessMessage> {
  let org_id = bson::oid::ObjectId::from_str(&org_id)?;

  GeneralPermission::Organization
    .verify_auth(db, auth, &org_id, StandardPermission::ReadWrite)
    .await?;

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
  let filename = format!("{}.{}", org_id.to_string(), extension);
  let bytes = profile_picture.get_image_bytes()?;
  // Save file in S3
  let s3 = CommonS3Bucket::new(OrganizationLogo, s3_config);
  let content_type_str = content_type.to_string();
  s3.add(&filename, bytes.as_slice(), content_type_str)
    .await?;
  // Update profile public url
  let org_collection = CommonCollection::<Organization>::new(db);
  let url = s3.get_public_url(&filename);
  org_collection.update_logo_url(&org_id, &Some(url)).await?;
  custom_message(Status::Ok, "Your profile picture is now updated")
}
