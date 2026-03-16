mod common;

use sdd_coverage::coverage::compute_coverage_status;
use sdd_coverage::models::{Annotation, AnnotationType, CoverageStatus, RequirementType};

// @req FR-COV-001
#[test]
fn covered_when_has_impl_and_test() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Covered
    );
}

// @req FR-COV-001
#[test]
fn partial_when_has_impl_but_no_test() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations = vec![common::make_annotation("FR-COV-001", AnnotationType::Impl)];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Partial
    );
}

// @req FR-COV-001
#[test]
fn missing_when_no_impl_annotations() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations: Vec<Annotation> = vec![];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Missing
    );
}

// @req FR-COV-001
#[test]
fn missing_when_only_test_annotations() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations = vec![common::make_annotation("FR-COV-001", AnnotationType::Test)];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Missing
    );
}

// @req FR-COV-001
#[test]
fn covered_with_multiple_impl_and_test() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Covered
    );
}

// @req FR-COV-001
#[test]
fn ignores_annotations_for_other_requirements() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations = vec![
        common::make_annotation("FR-COV-002", AnnotationType::Impl),
        common::make_annotation("FR-COV-002", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Missing
    );
}

// @req FR-COV-001
#[test]
fn mixed_annotations_only_counts_matching_requirement() {
    let req = common::make_requirement("FR-COV-001", RequirementType::FR);
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-002", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Partial
    );
}
