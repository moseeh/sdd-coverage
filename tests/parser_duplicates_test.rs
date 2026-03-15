use std::io::Write;

use sdd_coverage::error::ParseError;
use sdd_coverage::parser::{parse_requirements, parse_tasks};
use tempfile::NamedTempFile;
use tempfile::TempDir;

fn write_yaml(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

// @req FR-PARSE-004
#[test]
fn rejects_duplicate_requirement_ids() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-DUP-001
    type: FR
    title: First
    description: First requirement
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: FR-DUP-001
    type: FR
    title: Duplicate
    description: Duplicate requirement
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let err = parse_requirements(file.path()).unwrap_err();
    assert!(matches!(err, ParseError::DuplicateId { .. }));
    let msg = err.to_string();
    assert!(msg.contains("FR-DUP-001"));
}

// @req FR-PARSE-004
#[test]
fn rejects_duplicate_task_ids() {
    let dir = TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    let tasks_path = dir.path().join("tasks.yaml");
    std::fs::write(
        &tasks_path,
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: First
    status: open
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: TASK-001
    requirementId: FR-TEST-001
    title: Duplicate
    status: done
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    )
    .unwrap();

    let err = parse_tasks(&req_path).unwrap_err();
    assert!(matches!(err, ParseError::DuplicateId { .. }));
    let msg = err.to_string();
    assert!(msg.contains("TASK-001"));
}

// @req FR-PARSE-004
#[test]
fn accepts_unique_requirement_ids() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-UNQ-001
    type: FR
    title: First
    description: First
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: FR-UNQ-002
    type: FR
    title: Second
    description: Second
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let reqs = parse_requirements(file.path()).unwrap();
    assert_eq!(reqs.len(), 2);
}

// @req FR-PARSE-004
#[test]
fn accepts_unique_task_ids() {
    let dir = TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    let tasks_path = dir.path().join("tasks.yaml");
    std::fs::write(
        &tasks_path,
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: First
    status: open
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: TASK-002
    requirementId: FR-TEST-001
    title: Second
    status: done
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    )
    .unwrap();

    let tasks = parse_tasks(&req_path).unwrap();
    assert_eq!(tasks.len(), 2);
}

// @req FR-PARSE-004
#[test]
fn duplicate_error_includes_file_path() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-DUP-002
    type: FR
    title: First
    description: First
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: FR-DUP-002
    type: AR
    title: Second
    description: Second
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let err = parse_requirements(file.path()).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains(file.path().to_str().unwrap()),
        "error should contain file path: {}",
        msg
    );
}
