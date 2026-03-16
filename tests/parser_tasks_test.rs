use std::path::Path;

use sdd_coverage::models::TaskStatus;
use sdd_coverage::parser::parse_tasks;
use tempfile::TempDir;

// @req FR-PARSE-002
fn write_tasks_fixture(content: &str) -> TempDir {
    let dir = TempDir::new().unwrap();
    let tasks_path = dir.path().join("tasks.yaml");
    std::fs::write(&tasks_path, content).unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    dir
}

// @req FR-PARSE-002
#[test]
fn parse_valid_tasks_from_fixture() {
    let req_path = Path::new("fixtures/valid_project/requirements.yaml");
    let tasks = parse_tasks(req_path).unwrap();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, "TASK-001");
    assert_eq!(tasks[0].requirement_id, "FR-TEST-001");
    assert_eq!(tasks[0].status, TaskStatus::Done);
    assert_eq!(tasks[1].id, "TASK-002");
    assert_eq!(tasks[1].status, TaskStatus::Open);
    assert_eq!(tasks[1].assignee, Some("alice".to_string()));
    assert_eq!(tasks[2].id, "TASK-003");
    assert_eq!(tasks[2].status, TaskStatus::InProgress);
    assert_eq!(tasks[2].assignee, None);
}

// @req FR-PARSE-002
#[test]
fn parse_validates_all_required_fields() {
    let req_path = Path::new("fixtures/valid_project/requirements.yaml");
    let tasks = parse_tasks(req_path).unwrap();
    let task = &tasks[0];
    assert_eq!(task.id, "TASK-001");
    assert_eq!(task.requirement_id, "FR-TEST-001");
    assert_eq!(task.title, "Implement feature");
    assert_eq!(task.status, TaskStatus::Done);
    assert!(task.created_at.to_rfc3339().contains("2026-03-15"));
    assert!(task.updated_at.to_rfc3339().contains("2026-03-15"));
}

// @req FR-PARSE-002
#[test]
fn parse_empty_tasks_list() {
    let dir = write_tasks_fixture("tasks: []\n");
    let req_path = dir.path().join("requirements.yaml");
    let tasks = parse_tasks(&req_path).unwrap();
    assert!(tasks.is_empty());
}

// @req FR-PARSE-002
#[test]
fn parse_rejects_invalid_status() {
    let dir = write_tasks_fixture(
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: Test
    status: invalid
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let req_path = dir.path().join("requirements.yaml");
    let result = parse_tasks(&req_path);
    assert!(result.is_err());
}

// @req FR-PARSE-002
#[test]
fn parse_rejects_missing_required_field() {
    let dir = write_tasks_fixture(
        r#"
tasks:
  - id: TASK-001
    title: Test
    status: open
"#,
    );
    let req_path = dir.path().join("requirements.yaml");
    let result = parse_tasks(&req_path);
    assert!(result.is_err());
}

// @req FR-PARSE-002
#[test]
fn parse_rejects_invalid_timestamp() {
    let dir = write_tasks_fixture(
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: Test
    status: open
    createdAt: "not-a-date"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let req_path = dir.path().join("requirements.yaml");
    let result = parse_tasks(&req_path);
    assert!(result.is_err());
}

// @req FR-PARSE-002
#[test]
fn parse_rejects_missing_tasks_file() {
    let dir = TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();
    let result = parse_tasks(&req_path);
    assert!(result.is_err());
}

// @req FR-PARSE-002
#[test]
fn parse_assignee_is_optional() {
    let dir = write_tasks_fixture(
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: Without assignee
    status: open
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
  - id: TASK-002
    requirementId: FR-TEST-001
    title: With assignee
    status: open
    assignee: bob
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );
    let req_path = dir.path().join("requirements.yaml");
    let tasks = parse_tasks(&req_path).unwrap();
    assert_eq!(tasks[0].assignee, None);
    assert_eq!(tasks[1].assignee, Some("bob".to_string()));
}
