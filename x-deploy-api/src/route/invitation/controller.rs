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
use x_deploy_common::db::organization_invitation::InvitationStatus;
use x_deploy_common::db::organization_member::OrganizationMember;
use x_deploy_common::db::query::organization_invitation::{
  query_organization_invitation_by_id, query_organization_invitation_of_user,
  query_organization_invitation_update,
};

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
) -> ApiResult<Vec<InvitationInfoResponse>> {
  let user_id = token.parse_id()?;
  let invitation_db =
    query_organization_invitation_of_user(db, &user_id).await?;
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
  let invitation =
    query_organization_invitation_by_id(db, &invitation_id, &user_id).await?;
  return match invitation {
    Some(invitation) => {
      if invitation.status != InvitationStatus::Pending {
        return custom_error(Status::BadRequest, "Invitation is not pending");
      }
      let new_status = match body.response {
        true => InvitationStatus::Accepted,
        false => InvitationStatus::Pending,
      };
      let new_org_user = OrganizationMember {
        id: ObjectId::new(),
        user_id: invitation.receiver.id,
        role: Some(invitation.role.id),
        is_owner: false,
        organization_id: invitation.organization.id,
      };
      let _ = new_org_user.insert(db).await?;
      let result =
        query_organization_invitation_update(db, &invitation_id, &new_status)
          .await?;
      if result.modified_count == 0 {
        return custom_error(
          Status::InternalServerError,
          "Failed to update invitation",
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
