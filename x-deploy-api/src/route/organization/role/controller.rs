use crate::guard::bearer_token::BearerToken;
use crate::route::organization::role::dto::CustomRoleInfoResponse;
use crate::route::ApiResult;
use mongodb::Database;
use rocket::State;

pub(crate) async fn all(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
) -> ApiResult<Vec<CustomRoleInfoResponse>> {
  todo!()
}

pub(crate) async fn get_by_id(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
  custom_role_id: &str,
) -> ApiResult<CustomRoleInfoResponse> {
  todo!()
}

pub(crate) async fn delete(
  database: &State<Database>,
  token: BearerToken,
  org_id: &str,
  custom_role_id: &str,
) -> ApiResult<()> {
  todo!()
}
