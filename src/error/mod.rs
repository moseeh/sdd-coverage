use std::fmt;
use std::path::PathBuf;

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

// @req FR-PARSE-003
#[derive(Debug)]
pub enum ParseError {
    FileNotFound {
        path: PathBuf,
        source: std::io::Error,
    },
    MalformedYaml {
        path: PathBuf,
        line: Option<usize>,
        message: String,
    },
    // @req FR-PARSE-004
    DuplicateId {
        id: String,
        path: PathBuf,
    },
    // @req FR-PARSE-005
    InvalidIdFormat {
        id: String,
        expected: &'static str,
        path: PathBuf,
    },
}

// @req FR-PARSE-003
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FileNotFound { path, source } => {
                write!(f, "File not found: {}: {}", path.display(), source)
            }
            ParseError::MalformedYaml {
                path,
                line,
                message,
            } => match line {
                Some(line) => {
                    write!(
                        f,
                        "Malformed YAML in {} at line {}: {}",
                        path.display(),
                        line,
                        message
                    )
                }
                None => {
                    write!(f, "Malformed YAML in {}: {}", path.display(), message)
                }
            },
            // @req FR-PARSE-004
            ParseError::DuplicateId { id, path } => {
                write!(f, "Duplicate ID '{}' in {}", id, path.display())
            }
            // @req FR-PARSE-005
            ParseError::InvalidIdFormat { id, expected, path } => {
                write!(
                    f,
                    "Invalid ID format '{}' in {}: expected {}",
                    id,
                    path.display(),
                    expected
                )
            }
        }
    }
}

// @req FR-ERR-001
impl IntoResponse for ParseError {
    fn into_response(self) -> axum::response::Response {
        let error_type = match &self {
            ParseError::FileNotFound { .. } => "file_not_found",
            ParseError::MalformedYaml { .. } => "malformed_yaml",
            ParseError::DuplicateId { .. } => "duplicate_id",
            ParseError::InvalidIdFormat { .. } => "invalid_id_format",
        };

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": error_type,
                "message": self.to_string()
            })),
        )
            .into_response()
    }
}

// @req FR-ERR-001
pub async fn fallback_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "not_found",
            "message": "The requested endpoint does not exist"
        })),
    )
}
