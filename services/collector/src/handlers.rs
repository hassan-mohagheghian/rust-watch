use axum::{Json, extract::State, http::StatusCode};

use crate::{dtos::*, services::IngestionService, state::AppState};

pub async fn ingest(
    State(state): State<AppState>,
    Json(dto): Json<IngestRequestDto>,
) -> Result<Json<IngestResponseDto>, (StatusCode, Json<ErrorResponseDto>)> {
    if let Err(err) = dto.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponseDto { error: err }),
        ));
    }

    let event = IngestionService::transform(dto);

    let _ = state.sender.send(event).await;

    Ok(Json(IngestResponseDto {
        status: "accepted".to_string(),
    }))
}
