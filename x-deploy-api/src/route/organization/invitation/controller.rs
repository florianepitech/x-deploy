use crate::db::query::organization::get_org_by_id_with_owner;
use crate::db::query::organization_invitation::query_organization_invitation_get_all;
use crate::guard::token::Token;
use crate::route::organization::invitation::dto::{
  InvitationInfoResponse, InvitationInfoUser,
};
use crate::route::{custom_error, ApiResponse};
use bson::oid;
use mongodb::Database;
use rocket::http::Status;
use rocket::State;
use std::str::FromStr;

pub(crate) async fn get_all(
  db: &State<Database>,
  token: Token,
  org_id: String,
) -> ApiResponse<Vec<InvitationInfoResponse>> {
  let user_id = token.parse_id()?;
  let org_id = match oid::ObjectId::from_str(&org_id) {
    Ok(id) => id,
    Err(_) => {
      return custom_error(
        Status::BadRequest,
        "Organization id is not a valid id",
      )
    }
  };
  let org = get_org_by_id_with_owner(db, &user_id, &org_id).await?;
  let db_result = query_organization_invitation_get_all(db, &org_id).await?;
  let result: Vec<InvitationInfoResponse> = Vec::new();
  todo!()
}
