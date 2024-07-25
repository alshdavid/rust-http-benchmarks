use std::io;
use std::net::SocketAddr;

use bytes::BufMut;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

async fn main_async() -> io::Result<()> {
  println!("http://localhost:{}", get_port());
  let addr = SocketAddr::from(([127, 0, 0, 1], get_port()));
  let listener = TcpListener::bind(addr).await?;

  while let Ok((mut stream, _)) = listener.accept().await {
    tokio::task::spawn(async move {
      let mut buf = Vec::<u8>::new();

      loop {
        let mut raw_headers = [httparse::EMPTY_HEADER; 16];
        let mut raw_request = httparse::Request::new(&mut raw_headers);

        match raw_request.parse(&buf) {
          Ok(httparse::Status::Complete(_body_start)) => {
            // Send back static response for now
            stream
              .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nHello World")
              .await
              .unwrap();

            buf.clear();
          }
          Ok(httparse::Status::Partial) => {
            let mut temp_buf = vec![0; 512];
            match stream.read(&mut temp_buf).await {
              Ok(0) => break,
              Ok(n) => buf.put(&temp_buf[0..n]),
              Err(err) => println!("err = {err:?}"),
            }
            continue;
          }
          Err(_) => {
            todo!();
          }
        };
      }
    });
  }

  Ok(())
}

fn main() {
  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .worker_threads(num_cpus::get_physical())
    .build()
    .unwrap();

  rt.block_on(main_async()).unwrap();
}

fn get_port() -> u16 {
  std::env::var("PORT")
    .unwrap_or("8080".to_string())
    .parse::<u16>()
    .unwrap()
}
