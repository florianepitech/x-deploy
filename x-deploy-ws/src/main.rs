#[macro_use]
extern crate rocket;

use rocket::futures::stream::StreamExt;
use rocket::http::ContentType;
use rocket::response::stream::TextStream;
use rocket::tokio::sync::mpsc;
use rocket::tokio::sync::Mutex;
use rocket::tokio::time::{interval, Duration};
use rocket_ws::{Config, Stream, WebSocket};
use std::sync::Arc;

#[get("/")]
fn index(ws: WebSocket) -> Stream!['static] {
  let ws = ws.config(Config {
    max_send_queue: Some(5),
    ..Default::default()
  });

  Stream! { ws =>
      for await message in ws {
          yield message?;
      }
  }
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .configure(rocket::Config {
      address: "0.0.0.0".parse().unwrap(),
      port: 8001,
      ..rocket::Config::default()
    })
    .mount("/", routes![index])
}
