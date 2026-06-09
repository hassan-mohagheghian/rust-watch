use protocol::{TelemetryEvent, TelemetryType};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    let event = TelemetryEvent {
        service_name: "agent-service".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        event_type: TelemetryType::Log,
        payload: "hello from agent".to_string(),
    };

    let res = client
        .post("Http://localhost:3000/ingest")
        .json(&event)
        .send()
        .await
        .unwrap();

    println!(
        "Agent sent event: {:?} with body={:?}",
        res.status(),
        res.text().await.unwrap()
    );
}
