use protocol::{TelemetryEvent, TelemetryType};

fn main() {
    let event = TelemetryEvent {
        service_name: "agent-service".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        event_type: TelemetryType::Log,
        payload: "hello from agent".to_string(),
    };

    println!("Generated event: {:?}", event);
}
