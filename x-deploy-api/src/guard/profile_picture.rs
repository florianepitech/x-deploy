use crate::route::ErrorMessage;
use request::FromRequest;
use rocket::http::ContentType;
use rocket::{outcome::Outcome, request, Request};

pub struct ProfilePicture {
  extension: String,
  data: Vec<u8>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ProfilePicture {
  type Error = ErrorMessage;

  async fn from_request(
    req: &'r Request<'_>
  ) -> request::Outcome<Self, Self::Error> {
    let authorized_content_type: Vec<ContentType> =
      vec![ContentType::JPEG, ContentType::PNG, ContentType::GIF];
    let content_type = match req.content_type() {
      Some(content_type) => content_type,
      None => {
        let message =
          ErrorMessage::new("Content-Type header must be present".to_string());
        return Outcome::Error((rocket::http::Status::BadRequest, message));
      }
    };
    let extension =
      authorized_content_type
        .iter()
        .find(|&authorized_content_type| {
          *authorized_content_type == *content_type
        });
    let extension = match extension {
      Some(extension) => extension,
      None => {
        let message =
          ErrorMessage::new("Content-Type not authorized".to_string());
        return Outcome::Error((rocket::http::Status::BadRequest, message));
      }
    };
    todo!("Implement ProfilePicture guard")
  }
}
