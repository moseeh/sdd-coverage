use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// @req FR-PARSE-001
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequirementType {
    FR,
    AR,
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
