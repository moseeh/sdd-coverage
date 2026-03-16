mod common;

use sdd_coverage::coverage::compute_coverage_summary;
use sdd_coverage::models::{Annotation, AnnotationType, RequirementType};

// @req FR-COV-002
#[test]
fn summary_with_all_covered() {
    let reqs = vec![
        common::make_requirement("FR-COV-001", RequirementType::FR),
        common::make_requirement("FR-COV-002", RequirementType::FR),
    ];
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
        common::make_annotation("FR-COV-002", AnnotationType::Impl),
        common::make_annotation("FR-COV-002", AnnotationType::Test),
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
        common::make_requirement("FR-COV-001", RequirementType::FR),
        common::make_requirement("FR-COV-002", RequirementType::FR),
        common::make_requirement("FR-COV-003", RequirementType::FR),
    ];
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
        common::make_annotation("FR-COV-002", AnnotationType::Impl),
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
        common::make_requirement("FR-COV-001", RequirementType::FR),
        common::make_requirement("FR-COV-002", RequirementType::FR),
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
        common::make_requirement("FR-COV-001", RequirementType::FR),
        common::make_requirement("FR-COV-002", RequirementType::FR),
        common::make_requirement("FR-COV-003", RequirementType::FR),
        common::make_requirement("FR-COV-004", RequirementType::FR),
    ];
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
        common::make_annotation("FR-COV-002", AnnotationType::Impl),
        common::make_annotation("FR-COV-002", AnnotationType::Test),
        common::make_annotation("FR-COV-003", AnnotationType::Impl),
    ];
    let summary = compute_coverage_summary(&reqs, &annotations);
    assert_eq!(summary.covered, 2);
    assert_eq!(summary.partial, 1);
    assert_eq!(summary.missing, 1);
    assert!((summary.coverage_percentage - 50.0).abs() < f64::EPSILON);
}
