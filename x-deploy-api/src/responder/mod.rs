use crate::route::Message;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, Request, Response};

impl<'r> Responder<'r, 'static> for Message {
    fn respond_to(self, _: &'r Request) -> response::Result<'static> {
        Response::build()
            .status(Status::NotFound)
            .sized_body(self.message.len(), std::io::Cursor::new(self.message))
            .ok()
    }
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<Message> {
    Json(Message::new("Unauthorized".to_string()))
}

#[catch(403)]
pub fn forbidden(req: &Request) -> Json<Message> {
    Json(Message::new("Forbidden".to_string()))
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<Message> {
    let message = format!("Sorry, '{}' is not a valid path.", req.uri());
    Json(Message::new(message))
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<Message> {
    Json(Message::new("Unprocessable Entity".to_string()))
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> Json<Message> {
    Json(Message::new("Internal Server Error".to_string()))
}
