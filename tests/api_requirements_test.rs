use std::collections::HashMap;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use chrono::{TimeZone, Utc};
use serde_json::Value;
use tokio::sync::RwLock;
use tower::ServiceExt;

use sdd_coverage::api::requirements::list_requirements;
use sdd_coverage::api::{AppState, SharedState};
use sdd_coverage::models::{
    Annotation, AnnotationStats, AnnotationType, HealthStatus, Requirement, RequirementStats,
    RequirementType, ScanResult, TaskStats,
};

fn make_scan_result() -> ScanResult {
    let reqs = vec![
        Requirement {
            id: "FR-COV-001".to_string(),
            req_type: RequirementType::FR,
            title: "First".to_string(),
            description: "First desc".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 16, 0, 0, 0).unwrap(),
        },
        Requirement {
            id: "AR-CLI-001".to_string(),
            req_type: RequirementType::AR,
            title: "Second".to_string(),
            description: "Second desc".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
        },
        Requirement {
            id: "FR-COV-002".to_string(),
            req_type: RequirementType::FR,
            title: "Third".to_string(),
            description: "Third desc".to_string(),
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 17, 0, 0, 0).unwrap(),
        },
    ];

    // FR-COV-001: impl + test => covered
    // AR-CLI-001: impl only => partial
    // FR-COV-002: nothing => missing
    let annotations = vec![
        Annotation {
            file: "src/main.rs".to_string(),
            line: 1,
            req_id: "FR-COV-001".to_string(),
            annotation_type: AnnotationType::Impl,
            snippet: "// @req".to_string(),
        },
        Annotation {
            file: "tests/test.rs".to_string(),
            line: 1,
            req_id: "FR-COV-001".to_string(),
            annotation_type: AnnotationType::Test,
            snippet: "// @req".to_string(),
        },
        Annotation {
            file: "src/config.rs".to_string(),
            line: 1,
            req_id: "AR-CLI-001".to_string(),
            annotation_type: AnnotationType::Impl,
            snippet: "// @req".to_string(),
        },
    ];

    ScanResult {
        requirements: reqs,
        tasks: vec![],
        annotations,
        orphan_annotations: vec![],
        orphan_tasks: vec![],
        requirement_stats: RequirementStats {
            total: 3,
            by_type: HashMap::new(),
            by_status: HashMap::new(),
        },
        annotation_stats: AnnotationStats {
            total: 3,
            impl_count: 2,
            test_count: 1,
            orphans: 0,
        },
        task_stats: TaskStats {
            total: 0,
            by_status: HashMap::new(),
            orphans: 0,
        },
        coverage_percentage: 33.33,
        warnings: vec![],
    }
}

fn make_state() -> SharedState {
    Arc::new(RwLock::new(AppState {
        scan_result: Some(make_scan_result()),
        health_status: HealthStatus::Healthy,
        last_scan_at: Some(Utc::now()),
    }))
}

fn make_app(state: SharedState) -> Router {
    Router::new()
        .route("/requirements", get(list_requirements))
        .with_state(state)
}

// @req FR-API-003
#[tokio::test]
async fn returns_all_requirements_with_status() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.len(), 3);

    // Default sort by id asc
    assert_eq!(json[0]["id"], "AR-CLI-001");
    assert_eq!(json[1]["id"], "FR-COV-001");
    assert_eq!(json[2]["id"], "FR-COV-002");

    // Check status fields
    assert_eq!(json[0]["status"], "partial");
    assert_eq!(json[1]["status"], "covered");
    assert_eq!(json[2]["status"], "missing");
}

// @req FR-API-003
#[tokio::test]
async fn filters_by_type() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements?type=FR")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.len(), 2);
    assert!(json.iter().all(|r| r["type"] == "FR"));
}

// @req FR-API-003
#[tokio::test]
async fn filters_by_coverage_status() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements?status=covered")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.len(), 1);
    assert_eq!(json[0]["id"], "FR-COV-001");
}

// @req FR-API-003
#[tokio::test]
async fn sorts_by_updated_at_desc() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements?sort=updatedAt&order=desc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.len(), 3);
    // FR-COV-002 updatedAt=2026-03-17, FR-COV-001=2026-03-16, AR-CLI-001=2026-03-15
    assert_eq!(json[0]["id"], "FR-COV-002");
    assert_eq!(json[1]["id"], "FR-COV-001");
    assert_eq!(json[2]["id"], "AR-CLI-001");
}

// @req FR-API-003
#[tokio::test]
async fn sorts_by_id_desc() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements?sort=id&order=desc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(json[0]["id"], "FR-COV-002");
    assert_eq!(json[1]["id"], "FR-COV-001");
    assert_eq!(json[2]["id"], "AR-CLI-001");
}

// @req FR-API-003
#[tokio::test]
async fn filters_type_and_status_combined() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements?type=FR&status=missing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Vec<Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(json.len(), 1);
    assert_eq!(json[0]["id"], "FR-COV-002");
}

// @req FR-API-003
#[tokio::test]
async fn returns_503_when_no_scan_data() {
    let state: SharedState = Arc::new(RwLock::new(AppState {
        scan_result: None,
        health_status: HealthStatus::Degraded,
        last_scan_at: None,
    }));
    let app = make_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}
