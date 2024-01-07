mod auth;
mod cmd;
mod error;
mod executor;

use crate::cmd::CmdArgs;
use crate::executor::execute;
use clap::Parser;
use log::{error, info};
use std::io::Write;

#[macro_export]
macro_rules! panic {
  ($($arg:tt)*) => {
    {
      error!("{}", format_args!($($arg)*));
      std::process::exit(1);
    }
  };
}

#[tokio::main]
async fn main() {
  env_logger::Builder::new()
    .filter_level(log::LevelFilter::Info)
    .format(|buf, record| {
      writeln!(buf, "[{}] {}", record.level(), record.args())
    })
    .init();
  let cmd_args = CmdArgs::parse();
  let result = execute(cmd_args).await;
  match result {
    Ok(message) => {
      info!("{}", message)
    }
    Err(err) => panic!("{}", err.message),
  }
}
