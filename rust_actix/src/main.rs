use actix_web::get;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;

fn get_port() -> u16 {
  std::env::var("PORT")
    .unwrap_or("8080".to_string())
    .parse::<u16>()
    .unwrap()
}

#[get("/")]
async fn index() -> impl Responder {
  "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("http://localhost:{}", get_port());
  HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", get_port()))?
    .run()
    .await
}
