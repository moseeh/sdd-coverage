mod common;

use std::path::PathBuf;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use serde_json::Value;
use tower::ServiceExt;

use sdd_coverage::error::{ParseError, fallback_handler};
use sdd_coverage::models::HealthStatus;

// @req FR-ERR-001
#[tokio::test]
async fn fallback_returns_404_json() {
    let state = common::make_app_state(HealthStatus::Degraded, None);
    let app = Router::new().fallback(fallback_handler).with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
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
    assert!(json["message"].is_string());
}

// @req FR-ERR-001
#[tokio::test]
async fn parse_error_file_not_found_returns_json() {
    let err = ParseError::FileNotFound {
        path: PathBuf::from("missing.yaml"),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
    };

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "file_not_found");
    assert!(json["message"].as_str().unwrap().contains("missing.yaml"));
}

// @req FR-ERR-001
#[tokio::test]
async fn parse_error_malformed_yaml_returns_json() {
    let err = ParseError::MalformedYaml {
        path: PathBuf::from("bad.yaml"),
        line: Some(5),
        message: "unexpected token".to_string(),
    };

    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "malformed_yaml");
    assert!(json["message"].as_str().unwrap().contains("line 5"));
}

// @req FR-ERR-001
#[tokio::test]
async fn parse_error_duplicate_id_returns_json() {
    let err = ParseError::DuplicateId {
        id: "FR-DUP-001".to_string(),
        path: PathBuf::from("requirements.yaml"),
    };

    let response = err.into_response();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "duplicate_id");
    assert!(json["message"].as_str().unwrap().contains("FR-DUP-001"));
}

// @req FR-ERR-001
#[tokio::test]
async fn parse_error_invalid_format_returns_json() {
    let err = ParseError::InvalidIdFormat {
        id: "BADID".to_string(),
        expected: "TYPE-DOMAIN-NNN",
        path: PathBuf::from("requirements.yaml"),
    };

    let response = err.into_response();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["error"], "invalid_id_format");
    assert!(json["message"].as_str().unwrap().contains("BADID"));
}

// @req FR-ERR-001
#[tokio::test]
async fn unknown_route_with_different_methods() {
    let app = Router::new()
        .route("/known", get(|| async { "ok" }))
        .fallback(fallback_handler);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/unknown")
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
}
