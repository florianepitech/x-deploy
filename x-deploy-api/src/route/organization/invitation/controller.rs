use crate::guard::token::Token;
use crate::route::{ApiResponse, SuccessMessage};
use mongodb::Database;
use rocket::State;

pub(crate) async fn get(
  db: &State<Database>,
  token: Token,
  org_id: String,
) -> ApiResponse<SuccessMessage> {
  todo!()
}
