mod common;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use serde_json::Value;
use tower::ServiceExt;

use sdd_coverage::api::SharedState;
use sdd_coverage::api::healthcheck::healthcheck;
use sdd_coverage::models::HealthStatus;

// @req FR-API-001
fn make_app(state: SharedState) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .with_state(state)
}

// @req FR-API-001
#[tokio::test]
async fn returns_healthy_status() {
    let state = common::make_app_state(HealthStatus::Healthy, None);
    let app = make_app(state);
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
}

// @req FR-API-001
#[tokio::test]
async fn returns_degraded_status() {
    let state = common::make_app_state(HealthStatus::Degraded, None);
    let app = make_app(state);
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
    assert_eq!(json["status"], "degraded");
}

// @req FR-API-001
#[tokio::test]
async fn returns_version_string() {
    let state = common::make_app_state(HealthStatus::Healthy, None);
    let app = make_app(state);
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
    assert!(json["version"].is_string());
}

// @req FR-API-001
#[tokio::test]
async fn returns_iso8601_timestamp() {
    let state = common::make_app_state(HealthStatus::Healthy, None);
    let app = make_app(state);
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
    let ts = json["timestamp"].as_str().unwrap();
    assert!(ts.contains("T"));
    assert!(ts.ends_with("Z") || ts.contains("+"));
}
