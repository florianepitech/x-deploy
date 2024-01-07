use crate::cipher::password::verify_password;
use crate::guard::token::Token;
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
use crate::CONFIG;
use bson::oid;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use x_deploy_common::db::organization::Organization;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::user::User;
use x_deploy_common::event::organization::send_organization_created_event;

pub(crate) async fn all(
  db: &State<Database>,
  token: Token,
) -> ApiResult<Vec<OrganizationInfoResponse>> {
  let id = token.parse_id()?;
  let orgs = OrganizationMember::get_all_org_of_user(db, &id).await?;
  let mut result: Vec<OrganizationInfoResponse> = Vec::new();
  for org in orgs {
    let org_info = OrganizationInfoResponse {
      id: org.organization.id.to_string(),
      name: org.organization.name,
      description: org.organization.description,
      website: org.organization.website,
      contact_email: org.organization.contact_email,
    };
    result.push(org_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;

  // Insert Organization in database
  let new_organization = Organization::new(
    body.name.clone(),
    body.description.clone(),
    body.website.clone(),
    body.contact_email.clone(),
  );
  new_organization.insert(&db).await?;
  let inserted_id = new_organization.id.clone();

  // Insert Organization member as owner
  let owner = OrganizationMember::new(inserted_id.clone(), user_id, true, None);
  owner.insert(&db).await?;

  let _ = send_organization_created_event(
    CONFIG.kafka_url.clone(),
    user_id,
    inserted_id,
  );
  info!("Inserted new organization with id: {}", inserted_id);
  custom_message(Status::Ok, "Organization created successfully")
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResult<OrganizationInfoResponse> {
  let orgs_id = match oid::ObjectId::parse_str(&id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Invalid organization id")
    }
  };
  let id = token.parse_id()?;
  let orgs = OrganizationMember::get_user_in_org(db, &orgs_id, &id).await?;
  return match orgs {
    None => custom_error(Status::NotFound, "Organization not found"),
    Some(orgs) => {
      let result = OrganizationInfoResponse {
        id: orgs.organization.id.to_string(),
        name: orgs.organization.name,
        description: orgs.organization.description,
        website: orgs.organization.website,
        contact_email: orgs.organization.contact_email,
      };
      custom_response(Status::Ok, result)
    }
  };
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<UpdateOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::parse_str(&id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Invalid organization id")
    }
  };
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;
  return match organization {
    None => custom_error(Status::NotFound, "Organization not found"),
    Some(organization) => {
      let mut org = organization.organization;
      org.name = body.name.clone();
      org.description = body.description.clone();
      org.website = body.website.clone();
      org.contact_email = body.contact_email.clone();
      let update_result = org.update(db).await?;
      if update_result.modified_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Failed to update organization",
        );
      }
      custom_message(Status::Ok, "Organization updated successfully")
    }
  };
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<DeleteOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let password = body.password.clone();
  let user = match User::find_with_id(db, &user_id).await? {
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
  let organization =
    OrganizationMember::get_user_in_org(db, &user_id, &org_id).await?;

  return match organization {
    None => custom_error(Status::NotFound, "Organization not found"),
    Some(organization) => {
      let result = organization.to_organization_member().delete(db).await?;
      // TODO: Delete member, custom role... etc
      if result.deleted_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Failed to delete organization",
        );
      }
      return custom_message(Status::Ok, "Organization deleted successfully");
    }
  };
}

pub(crate) async fn transfer(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<TransferOrganizationRequest>,
) -> ApiResult<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
