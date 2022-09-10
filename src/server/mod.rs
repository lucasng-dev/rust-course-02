#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

pub fn execute() {
  let default_path = format!("{}/src/server/public", env!("CARGO_MANIFEST_DIR"));
  let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
  println!("public path: {}", public_path);
  let server = Server::new("127.0.0.1:8080".to_string());
  server.run(WebsiteHandler::new(public_path));
}
