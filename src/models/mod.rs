use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// @req FR-PARSE-001
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequirementType {
    FR,
    AR,
}

// @req FR-PARSE-002
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Open,
    InProgress,
    Done,
}

// @req FR-PARSE-002
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub requirement_id: String,
    pub title: String,
    pub status: TaskStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// @req FR-SCAN-002
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationType {
    Impl,
    Test,
}

// @req FR-SCAN-001
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub file: String,
    pub line: usize,
    pub req_id: String,
    // @req FR-SCAN-002
    pub annotation_type: AnnotationType,
    // @req FR-SCAN-003
    pub snippet: String,
}

// @req FR-PARSE-001
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirement {
    pub id: String,
    #[serde(rename = "type")]
    pub req_type: RequirementType,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
