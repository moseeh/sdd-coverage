use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::coverage::compute_coverage_status;

use super::SharedState;

// @req FR-API-003
#[derive(Deserialize, Default)]
pub struct ListParams {
    #[serde(rename = "type")]
    pub req_type: Option<String>,
    pub status: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

// @req FR-API-003
pub async fn list_requirements(
    State(state): State<SharedState>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let state = state.read().await;

    let Some(ref result) = state.scan_result else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "no_scan_data",
                "message": "No scan has been completed yet"
            })),
        );
    };

    let mut items: Vec<serde_json::Value> = result
        .requirements
        .iter()
        .map(|req| {
            let status = compute_coverage_status(req, &result.annotations);
            json!({
                "id": req.id,
                "type": req.req_type,
                "title": req.title,
                "description": req.description,
                "status": status,
                "createdAt": req.created_at.to_rfc3339(),
                "updatedAt": req.updated_at.to_rfc3339()
            })
        })
        .collect();

    if let Some(ref type_filter) = params.req_type {
        items.retain(|r| {
            r["type"]
                .as_str()
                .is_some_and(|t| t.eq_ignore_ascii_case(type_filter))
        });
    }

    if let Some(ref status_filter) = params.status {
        items.retain(|r| {
            r["status"]
                .as_str()
                .is_some_and(|s| s.eq_ignore_ascii_case(status_filter))
        });
    }

    let sort_field = params.sort.as_deref().unwrap_or("id");
    let descending = params.order.as_deref() == Some("desc");

    items.sort_by(|a, b| {
        let cmp = match sort_field {
            "updatedAt" => {
                let a_val = a["updatedAt"].as_str().unwrap_or_default();
                let b_val = b["updatedAt"].as_str().unwrap_or_default();
                a_val.cmp(b_val)
            }
            _ => {
                let a_val = a["id"].as_str().unwrap_or_default();
                let b_val = b["id"].as_str().unwrap_or_default();
                a_val.cmp(b_val)
            }
        };
        if descending { cmp.reverse() } else { cmp }
    });

    (StatusCode::OK, Json(json!(items)))
}

// @req FR-API-004
pub async fn get_requirement(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let state = state.read().await;

    let Some(ref result) = state.scan_result else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "no_scan_data",
                "message": "No scan has been completed yet"
            })),
        );
    };

    let Some(req) = result.requirements.iter().find(|r| r.id == id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "not_found",
                "message": format!("Requirement '{}' not found", id)
            })),
        );
    };

    let status = compute_coverage_status(req, &result.annotations);

    let annotations: Vec<serde_json::Value> = result
        .annotations
        .iter()
        .filter(|a| a.req_id == id)
        .map(|a| {
            json!({
                "file": a.file,
                "line": a.line,
                "reqId": a.req_id,
                "type": a.annotation_type,
                "snippet": a.snippet
            })
        })
        .collect();

    let tasks: Vec<serde_json::Value> = result
        .tasks
        .iter()
        .filter(|t| t.requirement_id == id)
        .map(|t| {
            json!({
                "id": t.id,
                "requirementId": t.requirement_id,
                "title": t.title,
                "status": t.status,
                "createdAt": t.created_at.to_rfc3339(),
                "updatedAt": t.updated_at.to_rfc3339()
            })
        })
        .collect();

    (
        StatusCode::OK,
        Json(json!({
            "id": req.id,
            "type": req.req_type,
            "title": req.title,
            "description": req.description,
            "status": status,
            "createdAt": req.created_at.to_rfc3339(),
            "updatedAt": req.updated_at.to_rfc3339(),
            "annotations": annotations,
            "tasks": tasks
        })),
    )
}
