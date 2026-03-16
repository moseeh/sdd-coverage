use std::collections::HashMap;
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

use sdd_coverage::api::tasks::list_tasks;
use sdd_coverage::api::{AppState, ScanState, SharedState};
use sdd_coverage::config::ProjectConfig;
use sdd_coverage::models::{
    AnnotationStats, HealthStatus, Requirement, RequirementStats, RequirementType, ScanResult,
    Task, TaskStats, TaskStatus,
};

fn make_scan_result() -> ScanResult {
    let reqs = vec![Requirement {
        id: "FR-COV-001".to_string(),
        req_type: RequirementType::FR,
        title: "Test".to_string(),
        description: "Test".to_string(),
        created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
        updated_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
    }];

    let tasks = vec![
        Task {
            id: "TASK-001".to_string(),
            requirement_id: "FR-COV-001".to_string(),
            title: "First task".to_string(),
            status: TaskStatus::Done,
            assignee: None,
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 16, 0, 0, 0).unwrap(),
        },
        Task {
            id: "TASK-002".to_string(),
            requirement_id: "FR-COV-001".to_string(),
            title: "Second task".to_string(),
            status: TaskStatus::Open,
            assignee: None,
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
        },
        Task {
            id: "TASK-003".to_string(),
            requirement_id: "FR-ORPHAN-001".to_string(),
            title: "Orphan task".to_string(),
            status: TaskStatus::InProgress,
            assignee: None,
            created_at: Utc.with_ymd_and_hms(2026, 3, 15, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 3, 17, 0, 0, 0).unwrap(),
        },
    ];

    ScanResult {
        requirements: reqs,
        tasks,
        annotations: vec![],
        orphan_annotations: vec![],
        orphan_tasks: vec![],
        requirement_stats: RequirementStats {
            total: 1,
            by_type: HashMap::new(),
            by_status: HashMap::new(),
        },
        annotation_stats: AnnotationStats {
            total: 0,
            impl_count: 0,
            test_count: 0,
            orphans: 0,
        },
        task_stats: TaskStats {
            total: 3,
            by_status: HashMap::new(),
            orphans: 1,
        },
        coverage_percentage: 0.0,
        warnings: vec![],
    }
}

fn make_state() -> SharedState {
    Arc::new(RwLock::new(AppState {
        scan_result: Some(make_scan_result()),
        health_status: HealthStatus::Healthy,
        last_scan_at: Some(Utc::now()),
        scan_state: ScanState::Idle,
        scan_started_at: None,
        scan_completed_at: None,
        scan_duration_ms: None,
        config: ProjectConfig {
            requirements: PathBuf::from("r.yaml"),
            source: PathBuf::from("src"),
            tests: PathBuf::from("tests"),
        },
    }))
}

fn make_app(state: SharedState) -> Router {
    Router::new()
        .route("/tasks", get(list_tasks))
        .with_state(state)
}

// @req FR-API-006
#[tokio::test]
async fn returns_all_tasks_sorted_by_id() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks")
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
    assert_eq!(json[0]["id"], "TASK-001");
    assert_eq!(json[1]["id"], "TASK-002");
    assert_eq!(json[2]["id"], "TASK-003");
}

// @req FR-API-006
#[tokio::test]
async fn filters_by_status() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks?status=done")
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
    assert_eq!(json[0]["id"], "TASK-001");
    assert_eq!(json[0]["status"], "done");
}

// @req FR-API-006
#[tokio::test]
async fn filters_orphans_only() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks?orphans=true")
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
    assert_eq!(json[0]["id"], "TASK-003");
    assert_eq!(json[0]["requirementId"], "FR-ORPHAN-001");
}

// @req FR-API-006
#[tokio::test]
async fn sorts_by_updated_at_desc() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks?sort=updatedAt&order=desc")
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
    // TASK-003=2026-03-17, TASK-001=2026-03-16, TASK-002=2026-03-15
    assert_eq!(json[0]["id"], "TASK-003");
    assert_eq!(json[1]["id"], "TASK-001");
    assert_eq!(json[2]["id"], "TASK-002");
}

// @req FR-API-006
#[tokio::test]
async fn combines_status_and_orphan_filters() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/tasks?status=in_progress&orphans=true")
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
    assert_eq!(json[0]["id"], "TASK-003");
}
