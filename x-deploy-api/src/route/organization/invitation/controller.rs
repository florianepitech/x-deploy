use crate::guard::token::Token;
use crate::permission::general::{
  verify_general_permission, GeneralPermissionType,
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

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: &str,
) -> ApiResult<Vec<OrganizationInvitationInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  match OrganizationMember::get_user_in_org(db, &user_id, &org_id).await? {
    Some(org) => {
      verify_general_permission(
        org.role,
        &GeneralPermissionType::Members,
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

  let org_invitation =
    OrganizationInvitation::get_all_of_org(db, &org_id).await?;
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
  token: Token,
  org_id: &str,
  body: Json<NewOrganizationInvitationRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let role_id = body.role_id.to_object_id()?;
  match OrganizationMember::get_user_in_org(db, &user_id, &org_id).await? {
    Some(org) => {
      verify_general_permission(
        org.role,
        &GeneralPermissionType::Members,
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
  let user_target = match User::find_with_email(db, &body.email).await? {
    Some(user) => user,
    None => {
      return custom_error(
        Status::NotFound,
        "The user to invite does not exist",
      );
    }
  };

  // Check if the target user is not already in the organization
  let target =
    OrganizationMember::get_user_in_org(db, &user_target.id, &org_id).await?;
  if let Some(_) = target {
    return custom_error(
      Status::BadRequest,
      "The target user is already in the organization",
    );
  }

  // Check if role exist in organization
  let org_role =
    match OrganizationRole::get_of_org(db, &org_id, &role_id).await? {
      Some(role) => role,
      None => {
        return custom_error(
          Status::NotFound,
          "The role does not exist in the organization",
        );
      }
    };

  // Verify if the use was not already invited
  let invitation =
    OrganizationInvitation::get_of_user_of_org(db, &org_id, &user_target.id)
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
  org_invitation.insert(db).await?;
  custom_message(Status::Created, "Invitation sent")
}

pub(crate) async fn delete_invitation(
  db: &State<Database>,
  token: Token,
  org_id: &str,
  invitation_id: &str,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let org_id = org_id.to_object_id()?;
  let invitation_id = invitation_id.to_object_id()?;
  match OrganizationMember::get_user_in_org(db, &user_id, &org_id).await? {
    Some(org_user) => {
      verify_general_permission(
        org_user.role,
        &GeneralPermissionType::Members,
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
  let org_invitation =
    match OrganizationInvitation::find_by_id(db, &invitation_id).await? {
      Some(org_invitation) => org_invitation,
      None => {
        return custom_error(Status::NotFound, "The invitation does not exist");
      }
    };
  if org_invitation.status != InvitationStatus::Pending {
    return custom_error(Status::BadRequest, "The invitation is not pending");
  }
  let delete_result = org_invitation.delete(db).await?;
  if delete_result.deleted_count == 0 {
    return custom_error(
      Status::InternalServerError,
      "The invitation was not deleted",
    );
  }
  custom_message(Status::Ok, "The invitation was deleted")
}
