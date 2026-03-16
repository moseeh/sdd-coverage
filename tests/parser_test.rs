mod common;

use std::path::Path;

use sdd_coverage::models::RequirementType;
use sdd_coverage::parser::parse_requirements;

// @req FR-PARSE-001
#[test]
fn parse_valid_requirements_from_fixture() {
    let path = Path::new("fixtures/valid_requirements.yaml");
    let reqs = parse_requirements(path).unwrap();
    assert_eq!(reqs.len(), 2);
    assert_eq!(reqs[0].id, "FR-TEST-001");
    assert_eq!(reqs[0].req_type, RequirementType::FR);
    assert_eq!(reqs[0].title, "Test requirement");
    assert_eq!(reqs[0].description, "A test requirement");
    assert_eq!(reqs[1].id, "AR-TEST-001");
    assert_eq!(reqs[1].req_type, RequirementType::AR);
}

// @req FR-PARSE-001
#[test]
fn parse_validates_timestamps() {
    let path = Path::new("fixtures/valid_requirements.yaml");
    let reqs = parse_requirements(path).unwrap();
    let req = &reqs[0];
    assert!(req.created_at.to_rfc3339().contains("2026-03-15"));
    assert!(req.updated_at.to_rfc3339().contains("2026-03-15"));
}

// @req FR-PARSE-001
#[test]
fn parse_empty_requirements_list() {
    let path = Path::new("fixtures/empty_requirements.yaml");
    let reqs = parse_requirements(path).unwrap();
    assert!(reqs.is_empty());
}

// @req FR-PARSE-001
#[test]
fn parse_rejects_missing_field() {
    let file = common::write_yaml_fixture(
        r#"
requirements:
  - id: FR-TEST-001
    type: FR
    title: Test
"#,
    );

    let result = parse_requirements(file.path());
    assert!(result.is_err());
}

// @req FR-PARSE-001
#[test]
fn parse_rejects_invalid_type() {
    let file = common::write_yaml_fixture(
        r#"
requirements:
  - id: FR-TEST-001
    type: INVALID
    title: Test
    description: Test
    createdAt: "2026-03-15T00:00:00Z"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let result = parse_requirements(file.path());
    assert!(result.is_err());
}

// @req FR-PARSE-001
#[test]
fn parse_rejects_invalid_timestamp() {
    let file = common::write_yaml_fixture(
        r#"
requirements:
  - id: FR-TEST-001
    type: FR
    title: Test
    description: Test
    createdAt: "not-a-date"
    updatedAt: "2026-03-15T00:00:00Z"
"#,
    );

    let result = parse_requirements(file.path());
    assert!(result.is_err());
}

// @req FR-PARSE-001
#[test]
fn parse_rejects_missing_file() {
    let result = parse_requirements(Path::new("/nonexistent/file.yaml"));
    assert!(result.is_err());
}

// @req FR-PARSE-001
#[test]
fn parse_rejects_malformed_yaml() {
    let file = common::write_yaml_fixture("not: [valid: yaml: at: all");

    let result = parse_requirements(file.path());
    assert!(result.is_err());
}
