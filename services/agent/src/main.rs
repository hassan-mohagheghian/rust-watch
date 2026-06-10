use protocol::{TelemetryEvent, TelemetryType};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let mut counter: u64 = 0;

    let events_per_second = 1000;
    let interval_ms = 1000 / events_per_second;

    loop {
        counter += 1;
        let event = TelemetryEvent {
            service_name: "agent-service".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            event_type: TelemetryType::Log,
            payload: format!("event number {}", counter),
        };

        let res = client
            .post("http://localhost:3000/ingest")
            .json(&event)
            .send()
            .await;

        match res {
            Ok(_) => {
                println!("[AGENT] sent event {}", counter);
            }
            Err(e) => {
                println!("[AGENT] error: {}", e);
            }
        }

        sleep(Duration::from_millis(interval_ms as u64)).await;
    }
}
