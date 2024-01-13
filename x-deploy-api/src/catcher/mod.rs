use crate::route::ErrorMessage;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;

#[deprecated]
#[catch(401)]
pub fn unauthorized() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "You are not authorized to access this resource, or your session is invalid".to_string()
  ))
}

#[deprecated]
#[catch(403)]
pub fn forbidden() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "You don't have permission to access this resource".to_string(),
  ))
}

#[deprecated]
#[catch(404)]
pub fn not_found() -> Json<ErrorMessage> {
  let message = "Sorry the resource was not found".to_string();
  Json(ErrorMessage::new(message))
}

#[deprecated]
#[catch(422)]
pub fn unprocessable_entity() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "Unprocessable entity, verify the format of your json.".to_string(),
  ))
}

#[deprecated]
#[catch(500)]
pub fn internal_server_error() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "An internal server error occurred, please try again later".to_string(),
  ))
}

#[catch(default)]
pub fn default(
  status: Status,
  _: &Request,
) -> Json<ErrorMessage> {
  let message = status.reason().unwrap_or("Unknown Error").to_string();
  Json(ErrorMessage::new(message))
}
