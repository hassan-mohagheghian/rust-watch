use protocol::TelemetryEvent;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub sender: mpsc::Sender<TelemetryEvent>,
}
