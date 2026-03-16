use axum::Json;
use axum::extract::{Query, State};
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
