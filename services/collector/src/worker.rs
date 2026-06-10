use protocol::TelemetryEvent;
use tokio::sync::mpsc;

pub async fn start_worker(mut receiver: mpsc::Receiver<TelemetryEvent>) {
    println!("Worker started");

    while let Some(event) = receiver.recv().await {
        println!(
            "[WORKER] service={} payload={}",
            event.service_name, event.payload
        );
    }
}
