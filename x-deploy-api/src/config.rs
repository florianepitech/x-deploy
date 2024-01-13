use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
  // General
  pub(crate) app_name: String,
  pub(crate) jwt_secret: String,
  // MongoDB
  pub(crate) mongodb_url: String,
  pub(crate) mongodb_database: String,
  // Kafka
  pub(crate) kafka_url: Vec<String>,
  // Redis
  pub(crate) redis_url: String,
  // Limits
  pub(crate) max_organization_by_owner: u64,
  pub(crate) max_apikey_by_organization: u64,
  pub(crate) jwt_key_duration_in_minutes: u64,
  // CORS
  pub(crate) cors_allowed_origins: Vec<String>,
  pub(crate) cors_allowed_methods: Vec<String>,
  pub(crate) cors_allowed_headers: Vec<String>,
  // S3
  pub(crate) s3_endpoint: String,
  pub(crate) s3_bucket: String,
  pub(crate) s3_access_key: String,
  pub(crate) s3_secret_key: String,
  pub(crate) s3_region: String,
}

impl Config {
  pub(crate) fn from_rocket_config() -> Self {
    let figment = rocket::Config::figment();
    let config = figment.extract::<Config>();
    return match config {
      Ok(config) => config,
      Err(err) => panic!("Error while parsing config file: {}", err),
    };
  }
}
