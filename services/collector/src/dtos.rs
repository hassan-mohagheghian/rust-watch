use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IngestRequestDTO {
    pub service_name: String,
    pub timestamp: i64,
    pub payload: String,
}

impl IngestRequestDTO {
    pub fn validate(&self) -> Result<(), String> {
        if self.service_name.trim().is_empty() {
            return Err("service_name cannot be empty".to_string());
        }
        if self.payload.trim().is_empty() {
            return Err("payload cannot be empty".to_string());
        }
        if self.payload.len() > 10_000 {
            return Err("payload exceeds maximum size".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct IngestResponseDTO {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseDTO {
    pub error: String,
}
