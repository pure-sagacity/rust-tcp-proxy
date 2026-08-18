use tokio::net::TcpListener;
use rust_tcp_proxy::{LISTEN_ADDRESS};

#[tokio::main]
async fn main() {
    let _listener = TcpListener::bind(LISTEN_ADDRESS).await.expect("failed to bind to listen address");

    println!("Starting server at {LISTEN_ADDRESS}");

    println!("Hello, world!");
}