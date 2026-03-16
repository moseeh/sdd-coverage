use std::collections::{HashMap, HashSet};

use crate::config::ProjectConfig;
use crate::error::ParseError;
use crate::models::{
    Annotation, AnnotationStats, AnnotationType, CoverageStatus, CoverageSummary, Requirement,
    RequirementStats, ScanResult, Task, TaskStats,
};
use crate::parser::{parse_requirements, parse_tasks};
use crate::scanner::{find_orphan_annotations, find_orphan_tasks, scan_files};

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

// @req FR-COV-003
pub fn compute_requirement_stats(
    requirements: &[Requirement],
    annotations: &[Annotation],
) -> RequirementStats {
    let mut by_type: HashMap<String, usize> = HashMap::new();
    let mut by_status: HashMap<String, usize> = HashMap::new();

    for req in requirements {
        let type_key = format!("{:?}", req.req_type);
        *by_type.entry(type_key).or_insert(0) += 1;

        let status = compute_coverage_status(req, annotations);
        let status_key = match status {
            CoverageStatus::Covered => "covered",
            CoverageStatus::Partial => "partial",
            CoverageStatus::Missing => "missing",
        };
        *by_status.entry(status_key.to_string()).or_insert(0) += 1;
    }

    RequirementStats {
        total: requirements.len(),
        by_type,
        by_status,
    }
}

// @req FR-COV-003
pub fn compute_annotation_stats(
    annotations: &[Annotation],
    orphan_count: usize,
) -> AnnotationStats {
    let impl_count = annotations
        .iter()
        .filter(|a| a.annotation_type == AnnotationType::Impl)
        .count();
    let test_count = annotations
        .iter()
        .filter(|a| a.annotation_type == AnnotationType::Test)
        .count();

    AnnotationStats {
        total: annotations.len(),
        impl_count,
        test_count,
        orphans: orphan_count,
    }
}

// @req FR-COV-003
pub fn compute_task_stats(tasks: &[Task], orphan_count: usize) -> TaskStats {
    let mut by_status: HashMap<String, usize> = HashMap::new();

    for task in tasks {
        let key = match task.status {
            crate::models::TaskStatus::Open => "open",
            crate::models::TaskStatus::InProgress => "in_progress",
            crate::models::TaskStatus::Done => "done",
        };
        *by_status.entry(key.to_string()).or_insert(0) += 1;
    }

    TaskStats {
        total: tasks.len(),
        by_status,
        orphans: orphan_count,
    }
}

// @req FR-COV-003
pub fn run_scan(config: &ProjectConfig) -> Result<ScanResult, ParseError> {
    let requirements = parse_requirements(&config.requirements)?;
    let tasks = parse_tasks(&config.requirements)?;
    let (annotations, warnings) = scan_files(
        &config.source,
        &config.tests,
        config
            .requirements
            .parent()
            .unwrap_or(config.source.as_ref()),
    );

    let requirement_ids: HashSet<&str> = requirements.iter().map(|r| r.id.as_str()).collect();
    let orphan_annotations: Vec<Annotation> =
        find_orphan_annotations(&annotations, &requirement_ids)
            .into_iter()
            .cloned()
            .collect();
    let orphan_tasks: Vec<Task> = find_orphan_tasks(&tasks, &requirement_ids)
        .into_iter()
        .cloned()
        .collect();

    let summary = compute_coverage_summary(&requirements, &annotations);
    let requirement_stats = compute_requirement_stats(&requirements, &annotations);
    let annotation_stats = compute_annotation_stats(&annotations, orphan_annotations.len());
    let task_stats = compute_task_stats(&tasks, orphan_tasks.len());

    Ok(ScanResult {
        requirements,
        tasks,
        annotations,
        orphan_annotations,
        orphan_tasks,
        requirement_stats,
        annotation_stats,
        task_stats,
        coverage_percentage: summary.coverage_percentage,
        warnings,
    })
}
