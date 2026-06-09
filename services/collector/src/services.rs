use protocol::TelemetryEvent;

use crate::dtos::IngestRequestDTO;

pub struct IngestService;

impl IngestService {
    pub fn process(dto: IngestRequestDTO) -> TelemetryEvent {
        let event = TelemetryEvent {
            service_name: dto.service_name,
            timestamp: dto.timestamp,
            event_type: protocol::TelemetryType::Log,
            payload: dto.payload,
        };

        println!(
            "[INGESTED] service={} payload={}",
            event.service_name, event.payload
        );
        event
    }
}
