use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryType {
    Log,
    Metric,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub service_name: String,
    pub timestamp: i64,
    pub event_type: TelemetryType,
    pub payload: String,
}
