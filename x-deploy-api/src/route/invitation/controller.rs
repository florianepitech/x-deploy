use crate::guard::token::Token;
use crate::route::invitation::dto::{
  InvitationInfoOrganization, InvitationInfoResponse, InvitationInfoUser,
  InvitationResponseRequest,
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
use x_deploy_common::db::CommonCollection;

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
) -> ApiResult<Vec<InvitationInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_invitation_coll = CommonCollection::<OrganizationInvitation>::new(db);
  let invitation_db = org_invitation_coll.get_of_user(&user_id).await?;
  let mut result: Vec<InvitationInfoResponse> = Vec::new();
  for invitation in invitation_db {
    let to_add = InvitationInfoResponse {
      id: invitation.id.to_string(),
      organization: InvitationInfoOrganization {
        id: invitation.organization.id.to_string(),
        name: invitation.organization.name,
        description: invitation.organization.description,
        website: invitation.organization.website,
      },
      sender: InvitationInfoUser {
        id: invitation.sender.id.to_string(),
        firstname: invitation.sender.firstname,
        lastname: invitation.sender.lastname,
        email: invitation.sender.email.email,
      },
    };
    result.push(to_add);
  }
  custom_response(Status::Ok, result)
}

pub(crate) async fn response(
  db: &State<Database>,
  token: Token,
  invitation_id: String,
  body: Json<InvitationResponseRequest>,
) -> ApiResult<SuccessMessage> {
  let user_id = token.parse_id()?;
  let invitation_id: ObjectId = match ObjectId::from_str(&invitation_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Invalid invitation id, please try again",
      )
    }
  };
  let org_invitation_coll = CommonCollection::<OrganizationInvitation>::new(db);
  let invitation = org_invitation_coll
    .get_of_user_with_invitation_id(&user_id, &invitation_id)
    .await?;
  return match invitation {
    Some(invitation) => {
      if invitation.status != InvitationStatus::Pending {
        return custom_error(Status::BadRequest, "Invitation is not pending");
      }
      let new_status = match body.response {
        true => InvitationStatus::Accepted,
        false => InvitationStatus::Pending,
      };
      let org_member_coll = CommonCollection::<OrganizationMember>::new(db);
      let new_org_user = OrganizationMember {
        id: ObjectId::new(),
        user_id: invitation.receiver.id,
        role: Some(invitation.role.id),
        is_owner: false,
        organization_id: invitation.organization.id,
      };
      org_member_coll.insert_one(&new_org_user).await?;
      let result = org_invitation_coll
        .update_status(&invitation_id, &new_status)
        .await?;
      if result.modified_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Failed to update invitation status",
        );
      }
      custom_message(
        Status::Ok,
        "You have successfully responded to the invitation",
      )
    }
    None => custom_error(Status::NotFound, "Invitation not found"),
  };
}
