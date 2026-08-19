use tokio::net::TcpListener;
use rust_tcp_proxy::{LISTEN_ADDRESS, proxy};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(LISTEN_ADDRESS).await.expect("failed to bind to listen address");

    println!("Starting server at {LISTEN_ADDRESS}");

    loop {
        let (socket, addr) = match listener.accept().await {
            Ok(t) => t,
            Err(e) => {
            println!("failde to accept client connection");
            eprintln!("{e}");
            continue
        }};

        println!("[+] New client connection from {}", addr.ip());

        tokio::spawn(async move {
            if let Err(e) = proxy(socket).await {
                println!("failed to proxy client");
                eprintln!("{e}");
            };
        });
    }
}