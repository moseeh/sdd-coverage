use std::collections::HashSet;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::models::TaskStatus;

use super::SharedState;

// @req FR-API-006
#[derive(Deserialize, Default)]
pub struct ListParams {
    pub status: Option<String>,
    #[serde(default)]
    pub orphans: Option<bool>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

// @req FR-API-006
pub async fn list_tasks(
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

    let requirement_ids: HashSet<&str> =
        result.requirements.iter().map(|r| r.id.as_str()).collect();

    let mut items: Vec<serde_json::Value> = result
        .tasks
        .iter()
        .filter(|t| {
            if let Some(ref status_filter) = params.status {
                let matches = match t.status {
                    TaskStatus::Open => status_filter.eq_ignore_ascii_case("open"),
                    TaskStatus::InProgress => status_filter.eq_ignore_ascii_case("in_progress"),
                    TaskStatus::Done => status_filter.eq_ignore_ascii_case("done"),
                };
                if !matches {
                    return false;
                }
            }

            if params.orphans == Some(true) {
                return !requirement_ids.contains(t.requirement_id.as_str());
            }

            true
        })
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
