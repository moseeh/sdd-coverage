use crate::models::{Annotation, AnnotationType, CoverageStatus, CoverageSummary, Requirement};

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

// @req FR-COV-002
pub fn compute_coverage_summary(
    requirements: &[Requirement],
    annotations: &[Annotation],
) -> CoverageSummary {
    let total = requirements.len();
    let mut covered = 0;
    let mut partial = 0;
    let mut missing = 0;

    for req in requirements {
        match compute_coverage_status(req, annotations) {
            CoverageStatus::Covered => covered += 1,
            CoverageStatus::Partial => partial += 1,
            CoverageStatus::Missing => missing += 1,
        }
    }

    let coverage_percentage = if total == 0 {
        0.0
    } else {
        (covered as f64 / total as f64) * 100.0
    };

    CoverageSummary {
        total,
        covered,
        partial,
        missing,
        coverage_percentage,
    }
}
