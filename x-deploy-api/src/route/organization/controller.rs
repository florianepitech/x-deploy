use crate::cipher::password::verify_password;
use crate::db::organization::{Organization, ORGANIZATION_COLLECTION_NAME};
use crate::db::query::organization::{
  delete_organization_by_id, get_all_orgs_of_user, get_org_by_id_with_owner,
  insert_one_organization, update_organization_info,
  verify_number_of_created_organization,
};
use crate::db::query::user::get_user_from_db;
use crate::guard::token::Token;
use crate::route::organization::dto::{
  CreateOrganizationBody, OrganizationInfoResponse, UpdateOrganizationBody,
};
use crate::route::organization::dto::{
  DeleteOrganizationBody, TransferOrganizationBody,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResponse, SuccessMessage,
};
use bson::oid;
use mongodb::{Collection, Database};
use rocket::http::ext::IntoCollection;
use rocket::http::private::SmallVec;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub(crate) async fn all(
  db: &State<Database>,
  token: Token,
) -> ApiResponse<Vec<OrganizationInfoResponse>> {
  let id = token.parse_id()?;
  let orgs: Vec<Organization> = get_all_orgs_of_user(db, &id).await?;
  let mut result: Vec<OrganizationInfoResponse> = Vec::new();
  for org in orgs {
    let org_info = OrganizationInfoResponse {
      id: org.id.to_string(),
      name: org.name,
      description: org.description,
      website: org.website,
      contact_email: org.contact_email,
    };
    result.push(org_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn new(
  db: &State<Database>,
  token: Token,
  body: Json<CreateOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  let owner = oid::ObjectId::parse_str(&token.id).unwrap();
  verify_number_of_created_organization(db, &owner).await?;
  // Get objectId from token
  let new_organization = Organization::new(
    body.name.clone(),
    body.description.clone(),
    body.website.clone(),
    body.contact_email.clone(),
    owner,
  );
  let result = insert_one_organization(&db, &new_organization).await?;
  let inserted_id = result.inserted_id;
  info!("Inserted new organization with id: {}", inserted_id);
  custom_message(Status::Ok, "Organization created successfully")
}

pub(crate) async fn get_by_id(
  db: &State<Database>,
  token: Token,
  id: String,
) -> ApiResponse<OrganizationInfoResponse> {
  let orgs_id = match oid::ObjectId::parse_str(&id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Invalid organization id")
    }
  };
  let id = token.parse_id()?;
  let orgs = get_org_by_id_with_owner(db, &id, &orgs_id).await?;
  let result = OrganizationInfoResponse {
    id: orgs.id.to_string(),
    name: orgs.name,
    description: orgs.description,
    website: orgs.website,
    contact_email: orgs.contact_email,
  };
  return custom_response(Status::Ok, result);
}

pub(crate) async fn update(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<UpdateOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::parse_str(&id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(Status::BadRequest, "Invalid organization id")
    }
  };
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  update_organization_info(
    db.inner(),
    &organization.id,
    body.name.clone(),
    body.description.clone(),
    body.website.clone(),
    body.contact_email.clone(),
  )
  .await?;
  return custom_message(Status::Ok, "Organization updated successfully");
}

pub(crate) async fn delete(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<DeleteOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  let user_id = token.parse_id()?;
  let password = body.password.clone();
  let user = get_user_from_db(db, &user_id).await?;
  let verify_password = user.verify_password(password.as_str())?;
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
  let organization = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let result = delete_organization_by_id(db.inner(), &organization.id).await?;
  if result.deleted_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "Failed to delete organization",
    );
  }
  return custom_message(Status::Ok, "Organization deleted successfully");
}

pub(crate) async fn transfer(
  db: &State<Database>,
  token: Token,
  id: String,
  body: Json<TransferOrganizationBody>,
) -> ApiResponse<SuccessMessage> {
  // let organization = get_organization_by_id!(db, id).await?;
  return custom_message(Status::NotImplemented, "Not implemented");
}
