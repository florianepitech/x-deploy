use crate::guard::bearer_token::BearerToken;
use crate::permission::general::GeneralPermission;
use crate::route::organization::invitation::dto::{
  NewOrganizationInvitationRequest, OrganizationInvitationInfoResponse,
  OrganizationInvitationInfoUser,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::str::FromStr;
use x_deploy_common::db::organization_invitation::{
  InvitationStatus, OrganizationInvitation,
};
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::{
  OrganizationRole, StandardPermission,
};
use x_deploy_common::db::user::User;
use x_deploy_common::db::CommonCollection;

pub async fn get_all(
  db: &State<Database>,
  auth: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<OrganizationInvitationInfoResponse>> {
  let user_id = auth.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;

  GeneralPermission::Members
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::Read)
    .await?;

  let org_invitation_coll = CommonCollection::<OrganizationInvitation>::new(db);
  let org_invitation = org_invitation_coll.get_all_of_org(&org_id).await?;
  let mut result = Vec::new();
  for invitation in org_invitation {
    let invitation_info = invitation.into();
    result.push(invitation_info);
  }
  return custom_response(Status::Ok, result);
}

pub async fn invite_user(
  db: &State<Database>,
  auth: BearerToken,
  org_id: &str,
  body: Json<NewOrganizationInvitationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = auth.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let role_id = ObjectId::from_str(&body.role_id)?;

  GeneralPermission::Members
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  // Verify if the user to invite exists
  let user_coll = CommonCollection::<User>::new(db);
  let user_target = match user_coll.find_with_email(&body.email).await? {
    Some(user) => user,
    None => {
      return custom_error(
        Status::NotFound,
        "The user to invite does not exist",
      );
    }
  };

  // Check if the target user is not already in the organization
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  let target = org_member_coll
    .get_user_in_org(&user_target.id, &org_id)
    .await?;
  if let Some(_) = target {
    return custom_error(
      Status::BadRequest,
      "The target user is already in the organization",
    );
  }

  // Check if role exist in organization
  let org_role_coll = CommonCollection::<OrganizationRole>::new(db);
  let org_role =
    match org_role_coll.get_with_id_and_org(&role_id, &org_id).await? {
      Some(role) => role,
      None => {
        return custom_error(
          Status::NotFound,
          "The role does not exist in the organization",
        );
      }
    };

  // Verify if the use was not already invited
  let org_invitation_coll = CommonCollection::<OrganizationInvitation>::new(db);
  let invitation = org_invitation_coll
    .get_of_user_of_org(&org_id, &user_target.id)
    .await?;
  if let Some(_) = invitation {
    return custom_error(
      Status::BadRequest,
      "The user was already invited to the organization",
    );
  }

  // Add invitation to database
  let org_invitation =
    OrganizationInvitation::new(org_id, user_id, user_target.id, org_role.id);
  org_invitation_coll.insert_one(&org_invitation).await?;
  custom_message(Status::Created, "Invitation sent")
}

pub async fn delete_invitation(
  db: &State<Database>,
  auth: BearerToken,
  org_id: &str,
  invitation_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = auth.parse_id()?;
  let org_id = ObjectId::from_str(org_id)?;
  let invitation_id = ObjectId::from_str(invitation_id)?;

  GeneralPermission::Members
    .verify_and_get(db, &user_id, &org_id, &StandardPermission::ReadWrite)
    .await?;

  // Retrieve the organization invitation
  let collection = CommonCollection::<OrganizationInvitation>::new(db);
  let invitation =
    match collection.get_by_id_of_org(&org_id, &invitation_id).await? {
      Some(org_invitation) => org_invitation,
      None => {
        return custom_error(Status::NotFound, "The invitation does not exist");
      }
    };
  if invitation.status != InvitationStatus::Pending {
    return custom_error(Status::BadRequest, "The invitation is not pending");
  }
  let delete_result = collection.delete_by_id(&invitation.id).await?;
  if delete_result.deleted_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "The invitation was not deleted",
    );
  }
  custom_message(Status::Ok, "The invitation was deleted")
}

pub async fn get_by_id(
  database: &State<Database>,
  auth: BearerToken,
  org_id: &str,
  invitation_id: &str,
) -> ApiResult<OrganizationInvitationInfoResponse> {
  let user_id = auth.parse_id().unwrap();
  let org_id = ObjectId::from_str(org_id).unwrap();
  let invitation_id = ObjectId::from_str(invitation_id).unwrap();

  GeneralPermission::Members
    .verify_and_get(database, &user_id, &org_id, &StandardPermission::Read)
    .await
    .unwrap();

  let collection = CommonCollection::<OrganizationInvitation>::new(database);
  let invitation = collection.get_by_id_of_org(&org_id, &invitation_id).await?;

  let invitation = match invitation {
    Some(invitation) => invitation,
    None => {
      return custom_error(Status::NotFound, "The invitation does not exist");
    }
  };
  let result = invitation.into();
  custom_response(Status::Ok, result)
}
