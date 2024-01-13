use crate::guard::bearer_token::BearerToken;
use crate::route::organization::invitation::dto::{
  NewOrganizationInvitationRequest, OrganizationInvitationInfoResponse,
};
use crate::route::{ApiResult, SuccessMessage};
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

mod controller;
pub mod dto;

#[utoipa::path(
    get,
    operation_id = "Get Invitation of Organization",
    path = "/organization/<org_id>/invitation",
    tag = "Organization Invitations",
    responses(
        (status = 200, description = "List of your current invitation", body = Vec<InvitationInfoResponse>),
    )
)]
#[get("/organization/<org_id>/invitation", format = "application/json")]
pub async fn get_all(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<OrganizationInvitationInfoResponse>> {
  controller::get_all(db, token, org_id).await
}

#[utoipa::path(
    post,
    operation_id = "Invite a user to your organization",
    path = "/organization/<org_id>/invitation",
    tag = "Organization Invitations",
    responses(
        (status = 200, description = "Invite a user to your organization", body = SuccessMessage),
    ),
    request_body = NewOrganizationInvitationRequest,
)]
#[post(
  "/organization/<org_id>/invitation",
  format = "application/json",
  data = "<body>"
)]
pub async fn new_invitation(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  body: Json<NewOrganizationInvitationRequest>,
) -> ApiResult<SuccessMessage> {
  controller::invite_user(db, token, org_id, body).await
}

#[utoipa::path(
    delete,
    operation_id = "Delete an invitation",
    path = "/organization/<org_id>/invitation/<invitation_id>",
    tag = "Organization Invitations",
    responses(
        (status = 200, description = "Delete an invitation", body = SuccessMessage),
    ),
)]
#[delete(
  "/organization/<org_id>/invitation/<invitation_id>",
  format = "application/json"
)]
pub async fn delete_invitation(
  db: &State<Database>,
  token: BearerToken,
  org_id: &str,
  invitation_id: &str,
) -> ApiResult<SuccessMessage> {
  controller::delete_invitation(db, token, org_id, invitation_id).await
}
