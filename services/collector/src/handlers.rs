use axum::{Json, http::StatusCode};

use crate::{
    dtos::{ErrorResponseDTO, IngestRequestDTO, IngestResponseDTO},
    services::IngestService,
};

pub async fn ingest(
    Json(dto): Json<IngestRequestDTO>,
) -> Result<Json<IngestResponseDTO>, (StatusCode, Json<ErrorResponseDTO>)> {
    if let Err(error) = dto.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponseDTO { error })));
    }

    IngestService::process(dto);

    Ok(Json(IngestResponseDTO {
        status: "accepted".into(),
    }))
}
