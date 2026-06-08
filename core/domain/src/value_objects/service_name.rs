use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub id: String,
    pub service_name: String,
    pub timestamp: i64,
    pub payload: String,
}
