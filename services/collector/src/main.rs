mod dtos;
mod handlers;
mod services;

use axum::{Router, routing::post};

use crate::handlers::ingest;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ingest", post(ingest));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Collector running on :3000");

    axum::serve(listener, app).await.unwrap();
}
