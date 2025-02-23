mod auth;
mod db;
mod errors;
mod routes;
use crate::routes::route::create_routes;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    let app = create_routes().await;
    dotenv().ok();
    let socketadd = env::var("SOCKET_ADDR").expect("SOCKET_ADDR must be set");

    let addr = socketadd.parse::<std::net::SocketAddr>().unwrap();
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
