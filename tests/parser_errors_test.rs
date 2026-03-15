use std::io::Write;
use std::path::Path;

use sdd_coverage::error::ParseError;
use sdd_coverage::parser::{parse_requirements, parse_tasks};
use tempfile::NamedTempFile;

fn write_yaml(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}

// @req FR-PARSE-003
#[test]
fn missing_file_reports_expected_path() {
    let path = Path::new("/expected/path/requirements.yaml");
    let err = parse_requirements(path).unwrap_err();
    let msg = err.to_string();
    assert!(matches!(err, ParseError::FileNotFound { .. }));
    assert!(msg.contains("/expected/path/requirements.yaml"));
}

// @req FR-PARSE-003
#[test]
fn malformed_yaml_reports_line_number() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-TEST-001
    type: FR
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: [invalid
"#,
    );

    let err = parse_requirements(file.path()).unwrap_err();
    match &err {
        ParseError::MalformedYaml { line, .. } => {
            assert!(line.is_some(), "expected line number in error");
        }
        _ => panic!("expected MalformedYaml, got {:?}", err),
    }
    let msg = err.to_string();
    assert!(msg.contains("at line"));
}

// @req FR-PARSE-003
#[test]
fn invalid_enum_reports_offending_value() {
    let file = write_yaml(
        r#"
requirements:
  - id: FR-TEST-001
    type: BADVALUE
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let err = parse_requirements(file.path()).unwrap_err();
    let msg = err.to_string();
    assert!(matches!(err, ParseError::MalformedYaml { .. }));
    assert!(
        msg.contains("BADVALUE"),
        "error should contain offending value: {}",
        msg
    );
}

// @req FR-PARSE-003
#[test]
fn invalid_task_status_reports_offending_value() {
    let file = write_yaml(
        r#"
tasks:
  - id: TASK-001
    requirementId: FR-TEST-001
    title: Test
    status: BADSTATUS
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let tasks_path = file.path().parent().unwrap().join("tasks.yaml");
    std::fs::copy(file.path(), &tasks_path).unwrap();
    let req_path = file.path().parent().unwrap().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();

    let err = parse_tasks(&req_path).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("BADSTATUS"),
        "error should contain offending value: {}",
        msg
    );
}

// @req FR-PARSE-003
#[test]
fn missing_tasks_file_reports_expected_path() {
    let dir = tempfile::TempDir::new().unwrap();
    let req_path = dir.path().join("requirements.yaml");
    std::fs::write(&req_path, "requirements: []\n").unwrap();

    let err = parse_tasks(&req_path).unwrap_err();
    let msg = err.to_string();
    assert!(matches!(err, ParseError::FileNotFound { .. }));
    assert!(msg.contains("tasks.yaml"));
}

// @req FR-PARSE-003
#[test]
fn no_panic_on_empty_file() {
    let file = write_yaml("");
    let result = parse_requirements(file.path());
    assert!(result.is_err());
}

// @req FR-PARSE-003
#[test]
fn no_panic_on_binary_content() {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(&[0x00, 0xFF, 0xFE, 0x89, 0x50, 0x4E, 0x47])
        .unwrap();
    let result = parse_requirements(file.path());
    assert!(result.is_err());
}

// @req FR-PARSE-003
#[test]
fn malformed_yaml_error_contains_file_path() {
    let file = write_yaml("{{{{not yaml");
    let err = parse_requirements(file.path()).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains(file.path().to_str().unwrap()),
        "error should contain file path: {}",
        msg
    );
}
