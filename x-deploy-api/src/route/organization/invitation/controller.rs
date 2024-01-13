use crate::guard::bearer_token::BearerToken;
use crate::permission::general::{
  verify_general_permission, GeneralPermission,
};
use crate::route::organization::invitation::dto::{
  NewOrganizationInvitationRequest, OrganizationInvitationInfoResponse,
  OrganizationInvitationInfoUser,
};
use crate::route::{
  custom_error, custom_message, custom_response, ApiResult, SuccessMessage,
};
use crate::utils::object_id::ToObjectId;
use chrono::{DateTime, Utc};
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use x_deploy_common::db::organization_invitation::{
  InvitationStatus, OrganizationInvitation,
};
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::organization_role::{
  OrganizationRole, StandardPermission,
};
use x_deploy_common::db::user::User;
use x_deploy_common::db::CommonCollection;

pub(crate) async fn get_all(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<OrganizationInvitationInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  match org_member_coll.get_user_in_org(&user_id, &org_id).await? {
    Some(org) => {
      verify_general_permission(
        org.role,
        &GeneralPermission::Members,
        &StandardPermission::Read,
      )?;
    }
    None => {
      return custom_error(
        Status::NotFound,
        "You are not a member of this organization",
      );
    }
  };
  let org_invitation_coll = CommonCollection::<OrganizationInvitation>::new(db);
  let org_invitation = org_invitation_coll.get_all_of_org(&org_id).await?;
  let mut result = Vec::new();
  for invitation in org_invitation {
    let send_at = invitation.id.timestamp().to_chrono().to_string();
    let response_at = match invitation.response_at {
      Some(response_at) => Some(response_at.to_string()),
      None => None,
    };
    let invitation_info = OrganizationInvitationInfoResponse {
      sender: OrganizationInvitationInfoUser {
        id: invitation.sender.id.to_string(),
        firstname: invitation.sender.firstname,
        lastname: invitation.sender.lastname,
        email: invitation.sender.email.email,
      },
      receiver: OrganizationInvitationInfoUser {
        id: invitation.receiver.id.to_string(),
        firstname: invitation.receiver.firstname,
        lastname: invitation.receiver.lastname,
        email: invitation.receiver.email.email,
      },
      status: invitation.status.to_string(),
      sent_at: send_at,
      response_at,
    };
    result.push(invitation_info);
  }
  return custom_response(Status::Ok, result);
}

pub(crate) async fn invite_user(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  body: Json<NewOrganizationInvitationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let role_id = body.role_id.to_object_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  match org_member_coll.get_user_in_org(&user_id, &org_id).await? {
    Some(org) => {
      verify_general_permission(
        org.role,
        &GeneralPermission::Members,
        &StandardPermission::ReadWrite,
      )?;
    }
    None => {
      return custom_error(
        Status::NotFound,
        "You are not a member of this organization",
      );
    }
  }

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
    match org_role_coll.get_with_id_of_org(&role_id, &org_id).await? {
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

pub(crate) async fn delete_invitation(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  invitation_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let invitation_id = invitation_id.to_object_id()?;
  let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
  match org_member_coll.get_user_in_org(&org_id, &user_id).await? {
    Some(org_user) => {
      verify_general_permission(
        org_user.role,
        &GeneralPermission::Members,
        &StandardPermission::ReadWrite,
      )?;
    }
    None => {
      return custom_error(
        Status::NotFound,
        "You are not a member of this organization",
      );
    }
  };
  // Retrieve the organization invitation
  let org_invitation_coll = CommonCollection::<OrganizationInvitation>::new(db);
  let org_invitation =
    match org_invitation_coll.get_by_id(&invitation_id).await? {
      Some(org_invitation) => org_invitation,
      None => {
        return custom_error(Status::NotFound, "The invitation does not exist");
      }
    };
  if org_invitation.status != InvitationStatus::Pending {
    return custom_error(Status::BadRequest, "The invitation is not pending");
  }
  let delete_result =
    org_invitation_coll.delete_by_id(&org_invitation.id).await?;
  if delete_result.deleted_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "The invitation was not deleted",
    );
  }
  custom_message(Status::Ok, "The invitation was deleted")
}
