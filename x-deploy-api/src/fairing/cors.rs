use crate::CONFIG;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::hyper::header::{
  ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
  ACCESS_CONTROL_ALLOW_ORIGIN,
};
use rocket::http::Header;
use rocket::{Request, Response};

pub(crate) struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
  fn info(&self) -> Info {
    Info {
      name: "Add headers to allow CORS",
      kind: Kind::Response,
    }
  }

  async fn on_response<'r>(
    &self,
    _request: &'r Request<'_>,
    response: &mut Response<'r>,
  ) {
    let cors_allowed_origins = CONFIG.cors_allowed_origins.join(",");
    let cors_allowed_methods = CONFIG.cors_allowed_methods.join(",");
    let cors_allowed_headers = CONFIG.cors_allowed_headers.join(",");
    response.set_header(Header::new(
      ACCESS_CONTROL_ALLOW_ORIGIN.to_string(),
      cors_allowed_origins,
    ));
    response.set_header(Header::new(
      ACCESS_CONTROL_ALLOW_METHODS.to_string(),
      cors_allowed_methods,
    ));
    response.set_header(Header::new(
      ACCESS_CONTROL_ALLOW_HEADERS.to_string(),
      cors_allowed_headers,
    ));
  }
}
