use serde::Deserialize;
use std::fs;

pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
  pub(crate) mongodb_url: String,
  pub(crate) mongodb_database: String,
  pub(crate) kafka_url: Vec<String>,
  pub(crate) redis_url: String,
  pub(crate) jwt_secret: String,
}

impl Config {
  pub(crate) fn from_config_file() -> Self {
    let contents = fs::read_to_string("config.toml").expect(
      format!("Error while reading file: {}", CONFIG_FILE_NAME).as_str(),
    );
    let config = toml::from_str::<Config>(contents.as_str());
    match config {
      Ok(config) => config,
      Err(err) => panic!("Error while parsing config file: {}", err),
    }
  }
}
