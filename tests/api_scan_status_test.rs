use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use chrono::{TimeZone, Utc};
use serde_json::Value;
use tokio::sync::RwLock;
use tower::ServiceExt;

use sdd_coverage::api::scan::get_scan_status;
use sdd_coverage::api::{AppState, ScanState, SharedState};
use sdd_coverage::config::ProjectConfig;
use sdd_coverage::models::HealthStatus;

fn dummy_config() -> ProjectConfig {
    ProjectConfig {
        requirements: PathBuf::from("r.yaml"),
        source: PathBuf::from("src"),
        tests: PathBuf::from("tests"),
    }
}

fn make_app(state: SharedState) -> Router {
    Router::new()
        .route("/scan", get(get_scan_status))
        .with_state(state)
}

// @req FR-API-008
#[tokio::test]
async fn returns_idle_when_no_scan_started() {
    let state: SharedState = Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: HealthStatus::Degraded,
        last_scan_at: None,
        scan_state: ScanState::Idle,
        scan_started_at: None,
        scan_completed_at: None,
        scan_duration_ms: None,
        scan_lock: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        config: dummy_config(),
    }));

    let app = make_app(state);
    let response = app
        .oneshot(Request::builder().uri("/scan").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "idle");
    assert!(json.get("startedAt").is_none());
    assert!(json.get("completedAt").is_none());
    assert!(json.get("duration").is_none());
}

// @req FR-API-008
#[tokio::test]
async fn returns_scanning_with_started_at() {
    let started = Utc.with_ymd_and_hms(2026, 3, 16, 10, 0, 0).unwrap();
    let state: SharedState = Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: HealthStatus::Degraded,
        last_scan_at: None,
        scan_state: ScanState::Scanning,
        scan_started_at: Some(started),
        scan_completed_at: None,
        scan_duration_ms: None,
        scan_lock: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        config: dummy_config(),
    }));

    let app = make_app(state);
    let response = app
        .oneshot(Request::builder().uri("/scan").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "scanning");
    assert!(json["startedAt"].is_string());
    assert!(json.get("completedAt").is_none());
}

// @req FR-API-008
#[tokio::test]
async fn returns_completed_with_all_fields() {
    let started = Utc.with_ymd_and_hms(2026, 3, 16, 10, 0, 0).unwrap();
    let completed = Utc.with_ymd_and_hms(2026, 3, 16, 10, 0, 1).unwrap();
    let state: SharedState = Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: HealthStatus::Healthy,
        last_scan_at: Some(completed),
        scan_state: ScanState::Completed,
        scan_started_at: Some(started),
        scan_completed_at: Some(completed),
        scan_duration_ms: Some(340),
        scan_lock: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        config: dummy_config(),
    }));

    let app = make_app(state);
    let response = app
        .oneshot(Request::builder().uri("/scan").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "completed");
    assert!(json["startedAt"].is_string());
    assert!(json["completedAt"].is_string());
    assert_eq!(json["duration"], 340);
}

// @req FR-API-008
#[tokio::test]
async fn returns_failed_status() {
    let started = Utc.with_ymd_and_hms(2026, 3, 16, 10, 0, 0).unwrap();
    let completed = Utc.with_ymd_and_hms(2026, 3, 16, 10, 0, 1).unwrap();
    let state: SharedState = Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: HealthStatus::Degraded,
        last_scan_at: None,
        scan_state: ScanState::Failed,
        scan_started_at: Some(started),
        scan_completed_at: Some(completed),
        scan_duration_ms: Some(100),
        scan_lock: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        config: dummy_config(),
    }));

    let app = make_app(state);
    let response = app
        .oneshot(Request::builder().uri("/scan").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "failed");
}
