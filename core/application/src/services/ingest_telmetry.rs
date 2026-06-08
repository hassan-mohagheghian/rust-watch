use crate::domain::entities::telemetry_event::TelemetryEvent;

pub struct IngestTelemetry;

impl IngestTelemetry {
    pub fn execute(event: TelemetryEvent) {
        println!(
            "[INGEST] service={} payload={}",
            event.service_name, event.payload
        );
    }
}
