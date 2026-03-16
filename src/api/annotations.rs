use std::collections::HashSet;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;

use crate::models::AnnotationType;

use super::SharedState;

// @req FR-API-005
#[derive(Deserialize, Default)]
pub struct ListParams {
    #[serde(rename = "type")]
    pub ann_type: Option<String>,
    #[serde(default)]
    pub orphans: Option<bool>,
}

// @req FR-API-005
pub async fn list_annotations(
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
        .annotations
        .iter()
        .filter(|a| {
            if let Some(ref type_filter) = params.ann_type {
                let matches = match a.annotation_type {
                    AnnotationType::Impl => type_filter.eq_ignore_ascii_case("impl"),
                    AnnotationType::Test => type_filter.eq_ignore_ascii_case("test"),
                };
                if !matches {
                    return false;
                }
            }

            if params.orphans == Some(true) {
                return !requirement_ids.contains(a.req_id.as_str());
            }

            true
        })
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

    items.sort_by(|a, b| {
        let file_cmp = a["file"]
            .as_str()
            .unwrap_or_default()
            .cmp(b["file"].as_str().unwrap_or_default());
        file_cmp.then_with(|| {
            a["line"]
                .as_u64()
                .unwrap_or(0)
                .cmp(&b["line"].as_u64().unwrap_or(0))
        })
    });

    (StatusCode::OK, Json(json!(items)))
}
