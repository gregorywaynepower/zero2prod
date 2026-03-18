//! main.rs

use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = "0";
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
