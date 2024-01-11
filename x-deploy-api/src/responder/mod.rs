use crate::route::{ErrorMessage, SuccessMessage};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, Request, Response};

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<ErrorMessage> {
  Json(ErrorMessage::new("You are not authorized to access this resource, or your session is invalid".to_string()))
}

#[catch(403)]
pub fn forbidden(req: &Request) -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "You don't have permission to access this resource".to_string(),
  ))
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorMessage> {
  let message = "Sorry the resource was not found".to_string();
  Json(ErrorMessage::new(message))
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "Unprocessable entity, verify the format of your json.".to_string(),
  ))
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> Json<ErrorMessage> {
  Json(ErrorMessage::new(
    "An internal server error occurred, please try again later".to_string(),
  ))
}
