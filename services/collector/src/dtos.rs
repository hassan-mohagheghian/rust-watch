use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct IngestRequestDTO {
    pub service_name: String,
    pub timestamp: i64,
    pub payload: String,
}

#[derive(Debug, Serialize)]
pub struct IngestResponseDTO {
    pub status: String,
}
