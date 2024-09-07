use hello_cargo::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = app();

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("Listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}