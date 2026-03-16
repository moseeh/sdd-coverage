use chrono::Utc;
use sdd_coverage::coverage::compute_coverage_status;
use sdd_coverage::models::{
    Annotation, AnnotationType, CoverageStatus, Requirement, RequirementType,
};

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

// @req FR-COV-001
#[test]
fn covered_when_has_impl_and_test() {
    let req = make_requirement("FR-COV-001");
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Covered
    );
}

// @req FR-COV-001
#[test]
fn partial_when_has_impl_but_no_test() {
    let req = make_requirement("FR-COV-001");
    let annotations = vec![make_annotation("FR-COV-001", AnnotationType::Impl)];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Partial
    );
}

// @req FR-COV-001
#[test]
fn missing_when_no_impl_annotations() {
    let req = make_requirement("FR-COV-001");
    let annotations: Vec<Annotation> = vec![];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Missing
    );
}

// @req FR-COV-001
#[test]
fn missing_when_only_test_annotations() {
    let req = make_requirement("FR-COV-001");
    let annotations = vec![make_annotation("FR-COV-001", AnnotationType::Test)];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Missing
    );
}

// @req FR-COV-001
#[test]
fn covered_with_multiple_impl_and_test() {
    let req = make_requirement("FR-COV-001");
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
        make_annotation("FR-COV-001", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Covered
    );
}

// @req FR-COV-001
#[test]
fn ignores_annotations_for_other_requirements() {
    let req = make_requirement("FR-COV-001");
    let annotations = vec![
        make_annotation("FR-COV-002", AnnotationType::Impl),
        make_annotation("FR-COV-002", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Missing
    );
}

// @req FR-COV-001
#[test]
fn mixed_annotations_only_counts_matching_requirement() {
    let req = make_requirement("FR-COV-001");
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-002", AnnotationType::Test),
    ];
    assert_eq!(
        compute_coverage_status(&req, &annotations),
        CoverageStatus::Partial
    );
}
