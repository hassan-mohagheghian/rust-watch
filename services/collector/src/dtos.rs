use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IngestRequestDto {
    pub service_name: String,
    pub timestamp: i64,
    pub event_type: String,
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct IngestResponseDto {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseDto {
    pub error: String,
}

impl IngestRequestDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.service_name.trim().is_empty() {
            return Err("service_name is required".to_string());
        }

        if self.payload.trim().is_empty() {
            return Err("payload is required".to_string());
        }

        Ok(())
    }
}
