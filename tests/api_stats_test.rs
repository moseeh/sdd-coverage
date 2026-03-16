mod common;

use std::collections::HashMap;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use serde_json::Value;
use tower::ServiceExt;

use sdd_coverage::api::SharedState;
use sdd_coverage::api::stats::get_stats;
use sdd_coverage::models::{
    AnnotationStats, HealthStatus, RequirementStats, ScanResult, TaskStats,
};

// @req FR-API-002
fn make_scan_result() -> ScanResult {
    let mut req_by_type = HashMap::new();
    req_by_type.insert("FR".to_string(), 3);
    req_by_type.insert("AR".to_string(), 1);

    let mut req_by_status = HashMap::new();
    req_by_status.insert("covered".to_string(), 2);
    req_by_status.insert("partial".to_string(), 1);
    req_by_status.insert("missing".to_string(), 1);

    let mut task_by_status = HashMap::new();
    task_by_status.insert("open".to_string(), 1);
    task_by_status.insert("done".to_string(), 2);

    ScanResult {
        requirements: vec![],
        tasks: vec![],
        annotations: vec![],
        orphan_annotations: vec![],
        orphan_tasks: vec![],
        requirement_stats: RequirementStats {
            total: 4,
            by_type: req_by_type,
            by_status: req_by_status,
        },
        annotation_stats: AnnotationStats {
            total: 10,
            impl_count: 6,
            test_count: 4,
            orphans: 1,
        },
        task_stats: TaskStats {
            total: 3,
            by_status: task_by_status,
            orphans: 0,
        },
        coverage_percentage: 50.0,
        warnings: vec![],
    }
}

// @req FR-API-002
fn make_app(state: SharedState) -> Router {
    Router::new()
        .route("/stats", get(get_stats))
        .with_state(state)
}

// @req FR-API-002
#[tokio::test]
async fn returns_stats_with_scan_data() {
    let state = common::make_app_state(HealthStatus::Healthy, Some(make_scan_result()));
    let app = make_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/stats")
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
    assert_eq!(json["requirements"]["total"], 4);
    assert_eq!(json["annotations"]["total"], 10);
    assert_eq!(json["tasks"]["total"], 3);
}

// @req FR-API-002
#[tokio::test]
async fn coverage_is_numeric() {
    let state = common::make_app_state(HealthStatus::Healthy, Some(make_scan_result()));
    let app = make_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json["coverage"].is_f64() || json["coverage"].is_i64());
}

// @req FR-API-002
#[tokio::test]
async fn returns_503_when_no_scan_data() {
    let state = common::make_app_state(HealthStatus::Degraded, None);
    let app = make_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/stats")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}
