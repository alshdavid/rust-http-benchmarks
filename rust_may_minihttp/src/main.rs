use std::io;
use std::net::SocketAddr;

use may_minihttp::HttpServer;
use may_minihttp::HttpService;
use may_minihttp::Request;
use may_minihttp::Response;

#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
  fn call(
    &mut self,
    _req: Request,
    rsp: &mut Response,
  ) -> io::Result<()> {
    rsp.body("Hello world");
    Ok(())
  }
}

fn main() {
  println!("http://localhost:{}", get_port());
  let addr = SocketAddr::from(([127, 0, 0, 1], get_port()));
  let server = HttpServer(HelloWorld).start(addr).unwrap();
  server.wait();
}

fn get_port() -> u16 {
  std::env::var("PORT")
    .unwrap_or("8080".to_string())
    .parse::<u16>()
    .unwrap()
}
