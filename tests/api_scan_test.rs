use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::post;
use serde_json::Value;
use tokio::sync::RwLock;
use tower::ServiceExt;

use sdd_coverage::api::scan::trigger_scan;
use sdd_coverage::api::{AppState, ScanLock, ScanState, SharedState};
use sdd_coverage::config::ProjectConfig;
use sdd_coverage::models::HealthStatus;

fn make_state() -> (SharedState, ScanLock) {
    let state = Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: HealthStatus::Degraded,
        last_scan_at: None,
        scan_state: ScanState::Idle,
        scan_started_at: None,
        scan_completed_at: None,
        scan_duration_ms: None,
        config: ProjectConfig {
            requirements: PathBuf::from("fixtures/scan_project/requirements.yaml"),
            source: PathBuf::from("fixtures/scan_project/src"),
            tests: PathBuf::from("fixtures/scan_project/tests"),
        },
    }));
    let lock = Arc::new(AtomicBool::new(false));
    (state, lock)
}

fn make_app(state: SharedState, lock: ScanLock) -> Router {
    Router::new()
        .route("/scan", post(trigger_scan))
        .with_state((state, lock))
}

// @req FR-API-007
#[tokio::test]
async fn returns_202_with_scanning_status() {
    let (state, lock) = make_state();
    let app = make_app(state, lock);
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/scan")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "scanning");
    assert!(json["startedAt"].is_string());
}

// @req FR-API-007
#[tokio::test]
async fn rejects_concurrent_scan_with_409() {
    let (state, lock) = make_state();
    // Simulate a scan already in progress
    lock.store(true, std::sync::atomic::Ordering::SeqCst);

    let app = make_app(state, lock);
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/scan")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "scan_in_progress");
    assert!(json["message"].is_string());
}

// @req FR-API-007
#[tokio::test]
async fn scan_completes_and_updates_state() {
    let (state, lock) = make_state();
    let app = make_app(state.clone(), lock.clone());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/scan")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    // Wait for the spawned scan to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let state = state.read().await;
    assert_eq!(state.scan_state, ScanState::Completed);
    assert_eq!(state.health_status, HealthStatus::Healthy);
    assert!(state.scan_result.is_some());
    assert!(state.last_scan_at.is_some());
    assert!(state.scan_completed_at.is_some());
    assert!(state.scan_duration_ms.is_some());
}

// @req FR-API-007
#[tokio::test]
async fn scan_releases_lock_after_completion() {
    let (state, lock) = make_state();
    let app = make_app(state, lock.clone());

    let _response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/scan")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    assert!(!lock.load(std::sync::atomic::Ordering::SeqCst));
}
