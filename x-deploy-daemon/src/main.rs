use crate::config::Config;
use env_logger::filter::Filter;
use lazy_static::lazy_static;
use x_deploy_common::event::user::UserEvent;

mod config;
mod error;
mod event;
mod cluster;

lazy_static! {
  pub(crate) static ref CONFIG: Config = Config::from_config_file();
}

#[tokio::main]
async fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::max())
    .init();
  let kafka_url = CONFIG.kafka_url.clone();
  let user_registered = tokio::spawn(async move {
    let result = UserEvent::listen_registered(
      kafka_url.clone(),
      event::user::listen_user_registered,
    )
    .await;
    if let Err(err) = result {
      log::error!("Error listening to user registered event {:?}", err);
    }
  });
  tokio::join!(user_registered);
}
