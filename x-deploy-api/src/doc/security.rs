use openapi::security::{ApiKey, ApiKeyValue};
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::yansi::Paint;
use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{openapi, Modify};

#[derive(Debug, Serialize)]
pub struct BearerSecurity;

impl Modify for BearerSecurity {
  fn modify(
    &self,
    openapi: &mut openapi::OpenApi,
  ) {
    if let Some(schema) = openapi.components.as_mut() {
      schema.add_security_scheme(
        "bearer",
        SecurityScheme::Http(
          HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .build(),
        ),
      );
    }
  }
}

#[derive(Debug, Serialize)]
pub struct ApiKeySecurity;

impl Modify for ApiKeySecurity {
  fn modify(
    &self,
    openapi: &mut openapi::OpenApi,
  ) {
    if let Some(schema) = openapi.components.as_mut() {
      let akh = ApiKey::Header(ApiKeyValue::new(AUTHORIZATION.to_string()));
      schema.add_security_scheme("apiKey", SecurityScheme::ApiKey(akh));
    }
  }
}
