use serde::Deserialize;
use std::fs;

pub(crate) const CONFIG_FILE_NAME: &str = "api-config.toml";

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
  pub(crate) mongodb_url: String,
  pub(crate) mongodb_database: String,
  pub(crate) kafka_url: Vec<String>,
  pub(crate) redis_url: String,
  pub(crate) jwt_secret: String,
  pub(crate) app_name: String,
  pub(crate) max_organization_by_owner: u64,
  pub(crate) max_apikey_by_organization: u64,
  pub(crate) jwt_key_duration_in_minutes: u64,
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
  pub(crate) fn from_config_file() -> Self {
    let contents = fs::read_to_string(CONFIG_FILE_NAME).expect(
      format!("Error while reading file: {}", CONFIG_FILE_NAME).as_str(),
    );
    let config = toml::from_str::<Config>(contents.as_str());
    match config {
      Ok(config) => config,
      Err(err) => panic!("Error while parsing config file: {}", err),
    }
  }
}
