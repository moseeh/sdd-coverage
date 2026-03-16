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

use sdd_coverage::api::requirements::get_requirement;
use sdd_coverage::api::{AppState, SharedState};
use sdd_coverage::models::{
    Annotation, AnnotationStats, AnnotationType, HealthStatus, Requirement, RequirementStats,
    RequirementType, ScanResult, Task, TaskStats, TaskStatus,
};

fn make_scan_result() -> ScanResult {
    let reqs = vec![Requirement {
        id: "FR-COV-001".to_string(),
        req_type: RequirementType::FR,
        title: "Coverage".to_string(),
        description: "Coverage desc".to_string(),
        created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2026, 3, 16, 0, 0, 0).unwrap(),
    }];

    let annotations = vec![
        Annotation {
            file: "src/coverage.rs".to_string(),
            line: 10,
            req_id: "FR-COV-001".to_string(),
            annotation_type: AnnotationType::Impl,
            snippet: "// @req FR-COV-001\nfn compute()".to_string(),
        },
        Annotation {
            file: "tests/coverage_test.rs".to_string(),
            line: 5,
            req_id: "FR-COV-001".to_string(),
            annotation_type: AnnotationType::Test,
            snippet: "// @req FR-COV-001\n#[test]".to_string(),
        },
        Annotation {
            file: "src/other.rs".to_string(),
            line: 1,
            req_id: "FR-COV-002".to_string(),
            annotation_type: AnnotationType::Impl,
            snippet: "// @req FR-COV-002".to_string(),
        },
    ];

    let tasks = vec![
        Task {
            id: "TASK-010".to_string(),
            requirement_id: "FR-COV-001".to_string(),
            title: "Implement coverage".to_string(),
            status: TaskStatus::Done,
            assignee: None,
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 16, 0, 0, 0).unwrap(),
        },
        Task {
            id: "TASK-020".to_string(),
            requirement_id: "FR-COV-002".to_string(),
            title: "Other task".to_string(),
            status: TaskStatus::Open,
            assignee: None,
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
        },
    ];

    ScanResult {
        requirements: reqs,
        tasks,
        annotations,
        orphan_annotations: vec![],
        orphan_tasks: vec![],
        requirement_stats: RequirementStats {
            total: 1,
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
            total: 2,
            by_status: HashMap::new(),
            orphans: 0,
        },
        coverage_percentage: 100.0,
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
        .route("/requirements/{id}", get(get_requirement))
        .with_state(state)
}

// @req FR-API-004
#[tokio::test]
async fn returns_requirement_with_linked_artifacts() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements/FR-COV-001")
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

    assert_eq!(json["id"], "FR-COV-001");
    assert_eq!(json["type"], "FR");
    assert_eq!(json["status"], "covered");

    let annotations = json["annotations"].as_array().unwrap();
    assert_eq!(annotations.len(), 2);
    assert!(annotations.iter().all(|a| a["reqId"] == "FR-COV-001"));

    let tasks = json["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0]["id"], "TASK-010");
    assert_eq!(tasks[0]["requirementId"], "FR-COV-001");
}

// @req FR-API-004
#[tokio::test]
async fn returns_404_for_unknown_requirement() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements/FR-UNKNOWN-999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "not_found");
    assert!(json["message"].as_str().unwrap().contains("FR-UNKNOWN-999"));
}

// @req FR-API-004
#[tokio::test]
async fn only_includes_matching_annotations_and_tasks() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements/FR-COV-001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Should not include FR-COV-002 annotations or TASK-020
    let annotations = json["annotations"].as_array().unwrap();
    assert!(annotations.iter().all(|a| a["reqId"] == "FR-COV-001"));

    let tasks = json["tasks"].as_array().unwrap();
    assert!(tasks.iter().all(|t| t["requirementId"] == "FR-COV-001"));
}

// @req FR-API-004
#[tokio::test]
async fn annotation_includes_all_fields() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/requirements/FR-COV-001")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let ann = &json["annotations"][0];
    assert!(ann["file"].is_string());
    assert!(ann["line"].is_number());
    assert!(ann["reqId"].is_string());
    assert!(ann["type"].is_string());
    assert!(ann["snippet"].is_string());
}
