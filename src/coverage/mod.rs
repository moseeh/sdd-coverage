use crate::models::{Annotation, AnnotationType, CoverageStatus, Requirement};

// @req FR-COV-001
pub fn compute_coverage_status(
    requirement: &Requirement,
    annotations: &[Annotation],
) -> CoverageStatus {
    let has_impl = annotations
        .iter()
        .any(|a| a.req_id == requirement.id && a.annotation_type == AnnotationType::Impl);
    let has_test = annotations
        .iter()
        .any(|a| a.req_id == requirement.id && a.annotation_type == AnnotationType::Test);

    match (has_impl, has_test) {
        (true, true) => CoverageStatus::Covered,
        (true, false) => CoverageStatus::Partial,
        _ => CoverageStatus::Missing,
    }
}
