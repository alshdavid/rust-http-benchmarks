extern crate tiny_http;

use std::net::SocketAddr;
use std::sync::Arc;

fn main() {
  let addr = SocketAddr::from(([127, 0, 0, 1], get_port()));
  let server = Arc::new(tiny_http::Server::http(addr).unwrap());
  println!("http://localhost:{}", get_port());

  for rq in server.incoming_requests() {
    let response = tiny_http::Response::from_string("hello world".to_string());
    let _ = rq.respond(response);
  }
}

fn get_port() -> u16 {
  std::env::var("PORT")
    .unwrap_or("8080".to_string())
    .parse::<u16>()
    .unwrap()
}
