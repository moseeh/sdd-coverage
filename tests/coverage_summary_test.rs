use chrono::Utc;
use sdd_coverage::coverage::compute_coverage_summary;
use sdd_coverage::models::{Annotation, AnnotationType, Requirement, RequirementType};

fn make_requirement(id: &str) -> Requirement {
    Requirement {
        id: id.to_string(),
        req_type: RequirementType::FR,
        title: "Test".to_string(),
        description: "Test".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn make_annotation(req_id: &str, annotation_type: AnnotationType) -> Annotation {
    Annotation {
        file: "src/main.rs".to_string(),
        line: 1,
        req_id: req_id.to_string(),
        annotation_type,
        snippet: "// @req".to_string(),
    }
}

// @req FR-COV-002
#[test]
fn summary_with_all_covered() {
    let reqs = vec![
        make_requirement("FR-COV-001"),
        make_requirement("FR-COV-002"),
    ];
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
        make_annotation("FR-COV-002", AnnotationType::Impl),
        make_annotation("FR-COV-002", AnnotationType::Test),
    ];
    let summary = compute_coverage_summary(&reqs, &annotations);
    assert_eq!(summary.total, 2);
    assert_eq!(summary.covered, 2);
    assert_eq!(summary.partial, 0);
    assert_eq!(summary.missing, 0);
    assert!((summary.coverage_percentage - 100.0).abs() < f64::EPSILON);
}

// @req FR-COV-002
#[test]
fn summary_with_mixed_statuses() {
    let reqs = vec![
        make_requirement("FR-COV-001"),
        make_requirement("FR-COV-002"),
        make_requirement("FR-COV-003"),
    ];
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
        make_annotation("FR-COV-002", AnnotationType::Impl),
    ];
    let summary = compute_coverage_summary(&reqs, &annotations);
    assert_eq!(summary.total, 3);
    assert_eq!(summary.covered, 1);
    assert_eq!(summary.partial, 1);
    assert_eq!(summary.missing, 1);
    assert!((summary.coverage_percentage - 100.0 / 3.0).abs() < 0.01);
}

// @req FR-COV-002
#[test]
fn summary_with_no_requirements() {
    let summary = compute_coverage_summary(&[], &[]);
    assert_eq!(summary.total, 0);
    assert_eq!(summary.covered, 0);
    assert_eq!(summary.partial, 0);
    assert_eq!(summary.missing, 0);
    assert!((summary.coverage_percentage - 0.0).abs() < f64::EPSILON);
}

// @req FR-COV-002
#[test]
fn summary_with_all_missing() {
    let reqs = vec![
        make_requirement("FR-COV-001"),
        make_requirement("FR-COV-002"),
    ];
    let annotations: Vec<Annotation> = vec![];
    let summary = compute_coverage_summary(&reqs, &annotations);
    assert_eq!(summary.total, 2);
    assert_eq!(summary.covered, 0);
    assert_eq!(summary.partial, 0);
    assert_eq!(summary.missing, 2);
    assert!((summary.coverage_percentage - 0.0).abs() < f64::EPSILON);
}

// @req FR-COV-002
#[test]
fn summary_percentage_is_covered_over_total() {
    let reqs = vec![
        make_requirement("FR-COV-001"),
        make_requirement("FR-COV-002"),
        make_requirement("FR-COV-003"),
        make_requirement("FR-COV-004"),
    ];
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
        make_annotation("FR-COV-002", AnnotationType::Impl),
        make_annotation("FR-COV-002", AnnotationType::Test),
        make_annotation("FR-COV-003", AnnotationType::Impl),
    ];
    let summary = compute_coverage_summary(&reqs, &annotations);
    assert_eq!(summary.covered, 2);
    assert_eq!(summary.partial, 1);
    assert_eq!(summary.missing, 1);
    assert!((summary.coverage_percentage - 50.0).abs() < f64::EPSILON);
}
