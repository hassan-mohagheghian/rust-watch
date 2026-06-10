use crate::dtos::IngestRequestDto;
use protocol::{TelemetryEvent, TelemetryType};

pub struct IngestionService;

impl IngestionService {
    pub fn transform(dto: IngestRequestDto) -> TelemetryEvent {
        TelemetryEvent {
            service_name: dto.service_name,
            timestamp: dto.timestamp,
            event_type: TelemetryType::Log,
            payload: dto.payload,
        }
    }
}
