mod dtos;
mod handlers;
mod services;
mod state;
mod worker;

use axum::{Router, routing::post};
use tokio::sync::mpsc;

use handlers::ingest;
use state::AppState;
use worker::start_worker;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(start_worker(rx));

    let state = AppState { sender: tx };

    let app = Router::new()
        .route("/ingest", post(ingest))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Collector running on :3000");

    axum::serve(listener, app).await.unwrap();
}
