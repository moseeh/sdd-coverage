use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

use super::SharedState;

// @req FR-API-002
pub async fn get_stats(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;

    let Some(ref result) = state.scan_result else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "no_scan_data",
                "message": "No scan has been completed yet"
            })),
        );
    };

    let last_scan_at = state
        .last_scan_at
        .map(|t| t.to_rfc3339())
        .unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "requirements": {
                "total": result.requirement_stats.total,
                "byType": result.requirement_stats.by_type,
                "byStatus": result.requirement_stats.by_status
            },
            "annotations": {
                "total": result.annotation_stats.total,
                "impl": result.annotation_stats.impl_count,
                "test": result.annotation_stats.test_count,
                "orphans": result.annotation_stats.orphans
            },
            "tasks": {
                "total": result.task_stats.total,
                "byStatus": result.task_stats.by_status,
                "orphans": result.task_stats.orphans
            },
            "coverage": result.coverage_percentage,
            "lastScanAt": last_scan_at
        })),
    )
}
