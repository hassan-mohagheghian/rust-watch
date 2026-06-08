use axum::{Json, Router, routing::post};
use protocol::TelemetryEvent;

async fn ingest(Json(payload): Json<TelemetryEvent>) -> &'static str {
    println!("received event: {:?}", payload);
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ingest", post(ingest));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Collector running on :3000");

    axum::serve(listener, app).await.unwrap();
}
