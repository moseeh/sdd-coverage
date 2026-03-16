use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use serde_json::Value;
use tokio::sync::RwLock;
use tower::ServiceExt;

use sdd_coverage::api::healthcheck::healthcheck;
use sdd_coverage::api::{AppState, ScanState, SharedState};
use sdd_coverage::config::ProjectConfig;
use sdd_coverage::models::HealthStatus;

fn make_state(status: HealthStatus) -> SharedState {
    Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: status,
        last_scan_at: None,
        scan_state: ScanState::Idle,
        scan_started_at: None,
        scan_completed_at: None,
        scan_duration_ms: None,
        scan_lock: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        config: ProjectConfig {
            requirements: PathBuf::from("r.yaml"),
            source: PathBuf::from("src"),
            tests: PathBuf::from("tests"),
        },
    }))
}

fn make_app(state: SharedState) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .with_state(state)
}

// @req FR-API-001
#[tokio::test]
async fn returns_healthy_status() {
    let app = make_app(make_state(HealthStatus::Healthy));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthcheck")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "healthy");
    assert!(json["version"].is_string());
    assert!(json["timestamp"].is_string());
}

// @req FR-API-001
#[tokio::test]
async fn returns_degraded_status() {
    let app = make_app(make_state(HealthStatus::Degraded));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthcheck")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "degraded");
}

// @req FR-API-001
#[tokio::test]
async fn returns_version_string() {
    let app = make_app(make_state(HealthStatus::Healthy));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthcheck")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["version"], env!("CARGO_PKG_VERSION"));
}

// @req FR-API-001
#[tokio::test]
async fn returns_iso8601_timestamp() {
    let app = make_app(make_state(HealthStatus::Healthy));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthcheck")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let timestamp = json["timestamp"].as_str().unwrap();
    // Verify it parses as a valid datetime
    chrono::DateTime::parse_from_rfc3339(timestamp).expect("timestamp should be valid RFC3339");
}
