use crate::route::ErrorMessage;
use rocket::serde::json::Json;

#[catch(401)]
pub fn unauthorized() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "You are not authorized to access this resource, or your session is invalid".to_string()
  ))
}

#[catch(403)]
pub fn forbidden() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "You don't have permission to access this resource".to_string(),
  ))
}

#[catch(404)]
pub fn not_found() -> Json<ErrorMessage> {
  let message = "Sorry the resource was not found".to_string();
  Json(ErrorMessage::new(message))
}

#[catch(422)]
pub fn unprocessable_entity() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "Unprocessable entity, verify the format of your json.".to_string(),
  ))
}

#[catch(500)]
pub fn internal_server_error() -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "An internal server error occurred, please try again later".to_string(),
  ))
}
