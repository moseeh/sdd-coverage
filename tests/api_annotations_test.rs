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

use sdd_coverage::api::annotations::list_annotations;
use sdd_coverage::api::{AppState, ScanState, SharedState};
use sdd_coverage::config::ProjectConfig;
use sdd_coverage::models::{
    Annotation, AnnotationStats, AnnotationType, HealthStatus, Requirement, RequirementStats,
    RequirementType, ScanResult, TaskStats,
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

    let annotations = vec![
        Annotation {
            file: "src/main.rs".to_string(),
            line: 10,
            req_id: "FR-COV-001".to_string(),
            annotation_type: AnnotationType::Impl,
            snippet: "// @req FR-COV-001".to_string(),
        },
        Annotation {
            file: "src/main.rs".to_string(),
            line: 20,
            req_id: "FR-ORPHAN-001".to_string(),
            annotation_type: AnnotationType::Impl,
            snippet: concat!("// @", "req FR-ORPHAN-001").to_string(),
        },
        Annotation {
            file: "tests/test.rs".to_string(),
            line: 5,
            req_id: "FR-COV-001".to_string(),
            annotation_type: AnnotationType::Test,
            snippet: "// @req FR-COV-001".to_string(),
        },
    ];

    ScanResult {
        requirements: reqs,
        tasks: vec![],
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
            orphans: 1,
        },
        task_stats: TaskStats {
            total: 0,
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
        .route("/annotations", get(list_annotations))
        .with_state(state)
}

// @req FR-API-005
#[tokio::test]
async fn returns_all_annotations_sorted_by_file_and_line() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/annotations")
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

    // Sorted by file then line
    assert_eq!(json[0]["file"], "src/main.rs");
    assert_eq!(json[0]["line"], 10);
    assert_eq!(json[1]["file"], "src/main.rs");
    assert_eq!(json[1]["line"], 20);
    assert_eq!(json[2]["file"], "tests/test.rs");
}

// @req FR-API-005
#[tokio::test]
async fn filters_by_type_impl() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/annotations?type=impl")
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
    assert!(json.iter().all(|a| a["type"] == "impl"));
}

// @req FR-API-005
#[tokio::test]
async fn filters_by_type_test() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/annotations?type=test")
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
    assert_eq!(json[0]["type"], "test");
}

// @req FR-API-005
#[tokio::test]
async fn filters_orphans_only() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/annotations?orphans=true")
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
    assert_eq!(json[0]["reqId"], "FR-ORPHAN-001");
}

// @req FR-API-005
#[tokio::test]
async fn combines_type_and_orphan_filters() {
    let app = make_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/annotations?type=impl&orphans=true")
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
    assert_eq!(json[0]["type"], "impl");
    assert_eq!(json[0]["reqId"], "FR-ORPHAN-001");
}
