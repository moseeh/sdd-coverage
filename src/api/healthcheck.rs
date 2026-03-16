use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;

use super::SharedState;

// @req FR-API-001
pub async fn healthcheck(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;
    let status = match state.health_status {
        crate::models::HealthStatus::Healthy => "healthy",
        crate::models::HealthStatus::Degraded => "degraded",
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": status,
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": Utc::now().to_rfc3339()
        })),
    )
}
