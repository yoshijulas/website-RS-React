mod auth;
mod db;
mod routes;
use crate::routes::route::create_routes;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = create_routes().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
