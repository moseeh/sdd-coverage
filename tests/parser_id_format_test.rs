use std::io::Write;

use sdd_coverage::error::ParseError;
use sdd_coverage::parser::{parse_requirements, parse_tasks};
use tempfile::{NamedTempFile, TempDir};

fn write_yaml(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

// @req FR-PARSE-005
#[test]
fn accepts_valid_requirement_id_formats() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-SCAN-001
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: AR-CLI-002
    type: AR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let reqs = parse_requirements(file.path()).unwrap();
    assert_eq!(reqs.len(), 2);
}

// @req FR-PARSE-005
#[test]
fn rejects_requirement_id_missing_type_prefix() {
    let file = write_yaml(
        r#"
requirements:
  - id: SCAN-001
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let err = parse_requirements(file.path()).unwrap_err();
    assert!(matches!(err, ParseError::InvalidIdFormat { .. }));
    assert!(err.to_string().contains("SCAN-001"));
}

// @req FR-PARSE-005
#[test]
fn rejects_requirement_id_lowercase_domain() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-scan-001
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let err = parse_requirements(file.path()).unwrap_err();
    assert!(matches!(err, ParseError::InvalidIdFormat { .. }));
}

// @req FR-PARSE-005
#[test]
fn rejects_requirement_id_missing_number() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-SCAN
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let err = parse_requirements(file.path()).unwrap_err();
    assert!(matches!(err, ParseError::InvalidIdFormat { .. }));
}

// @req FR-PARSE-005
#[test]
fn rejects_requirement_id_wrong_type() {
    let file = write_yaml(
        r#"
requirements:
  - id: XX-SCAN-001
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let err = parse_requirements(file.path()).unwrap_err();
    assert!(matches!(err, ParseError::InvalidIdFormat { .. }));
}

// @req FR-PARSE-005
#[test]
fn accepts_valid_task_id_format() {
    let dir = TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    std::fs::write(
        dir.path().join("tasks.yaml"),
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: Test
    status: open
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    )
    .unwrap();
    let tasks = parse_tasks(&req_path).unwrap();
    assert_eq!(tasks.len(), 1);
}

// @req FR-PARSE-005
#[test]
fn rejects_task_id_wrong_prefix() {
    let dir = TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    std::fs::write(
        dir.path().join("tasks.yaml"),
        r#"
tasks:
  - id: ITEM-001
    requirementId: FR-TEST-001
    title: Test
    status: open
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    )
    .unwrap();
    let err = parse_tasks(&req_path).unwrap_err();
    assert!(matches!(err, ParseError::InvalidIdFormat { .. }));
    assert!(err.to_string().contains("ITEM-001"));
}

// @req FR-PARSE-005
#[test]
fn rejects_task_id_missing_number() {
    let dir = TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    std::fs::write(
        dir.path().join("tasks.yaml"),
        r#"
tasks:
  - id: TASK-ABC
    requirementId: FR-TEST-001
    title: Test
    status: open
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    )
    .unwrap();
    let err = parse_tasks(&req_path).unwrap_err();
    assert!(matches!(err, ParseError::InvalidIdFormat { .. }));
}

// @req FR-PARSE-005
#[test]
fn error_message_includes_expected_format() {
    let file = write_yaml(
        r#"
requirements:
  - id: BADID
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let err = parse_requirements(file.path()).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("TYPE-DOMAIN-NNN"));
    assert!(msg.contains("BADID"));
}
