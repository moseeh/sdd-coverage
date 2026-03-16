use std::sync::atomic::Ordering;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;

use crate::coverage::run_scan;
use crate::models::HealthStatus;

use super::{ScanState, SharedState};

// @req FR-API-008
pub async fn get_scan_status(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;

    let status = match state.scan_state {
        ScanState::Idle => "idle",
        ScanState::Scanning => "scanning",
        ScanState::Completed => "completed",
        ScanState::Failed => "failed",
    };

    let mut response = json!({ "status": status });

    if let Some(started_at) = state.scan_started_at {
        response["startedAt"] = json!(started_at.to_rfc3339());
    }

    if let Some(completed_at) = state.scan_completed_at {
        response["completedAt"] = json!(completed_at.to_rfc3339());
    }

    if let Some(duration_ms) = state.scan_duration_ms {
        response["duration"] = json!(duration_ms);
    }

    (StatusCode::OK, Json(response))
}

// @req FR-API-007
pub async fn trigger_scan(State(state): State<SharedState>) -> impl IntoResponse {
    let lock = {
        let state = state.read().await;
        state.scan_lock.clone()
    };

    if lock
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "error": "scan_in_progress",
                "message": "A scan is already running"
            })),
        );
    }

    let started_at = Utc::now();

    {
        let mut state = state.write().await;
        state.scan_state = ScanState::Scanning;
        state.scan_started_at = Some(started_at);
        state.scan_completed_at = None;
        state.scan_duration_ms = None;
    }

    let config = {
        let state = state.read().await;
        state.config.clone()
    };

    let state_clone = state.clone();

    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(move || run_scan(&config)).await;

        let completed_at = Utc::now();
        let duration_ms = (completed_at - started_at).num_milliseconds();

        let mut state = state_clone.write().await;
        match result {
            Ok(Ok(scan_result)) => {
                state.scan_result = Some(scan_result);
                state.scan_state = ScanState::Completed;
                state.health_status = HealthStatus::Healthy;
            }
            _ => {
                state.scan_state = ScanState::Failed;
                state.health_status = HealthStatus::Degraded;
            }
        }
        state.scan_completed_at = Some(completed_at);
        state.scan_duration_ms = Some(duration_ms);
        state.last_scan_at = Some(completed_at);

        state.scan_lock.store(false, Ordering::SeqCst);
    });

    (
        StatusCode::ACCEPTED,
        Json(json!({
            "status": "scanning",
            "startedAt": started_at.to_rfc3339()
        })),
    )
}
