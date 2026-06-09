use axum::Json;

use crate::{
    dtos::{IngestRequestDTO, IngestResponseDTO},
    services::IngestService,
};

pub async fn ingest(Json(dto): Json<IngestRequestDTO>) -> Json<IngestResponseDTO> {
    IngestService::process(dto);

    Json(IngestResponseDTO {
        status: "accepted".to_string(),
    })
}
