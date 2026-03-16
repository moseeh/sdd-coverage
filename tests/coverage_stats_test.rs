mod common;

use sdd_coverage::coverage::{
    compute_annotation_stats, compute_requirement_stats, compute_task_stats,
};
use sdd_coverage::models::{AnnotationType, RequirementType, TaskStatus};

// @req FR-COV-003
#[test]
fn requirement_stats_counts_by_type() {
    let reqs = vec![
        common::make_requirement("FR-COV-001", RequirementType::FR),
        common::make_requirement("FR-COV-002", RequirementType::FR),
        common::make_requirement("AR-CLI-001", RequirementType::AR),
    ];
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
    ];
    let stats = compute_requirement_stats(&reqs, &annotations);
    assert_eq!(stats.total, 3);
    assert_eq!(stats.by_type.get("FR"), Some(&2));
    assert_eq!(stats.by_type.get("AR"), Some(&1));
}

// @req FR-COV-003
#[test]
fn requirement_stats_counts_by_coverage_status() {
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
    let stats = compute_requirement_stats(&reqs, &annotations);
    assert_eq!(stats.by_status.get("covered"), Some(&1));
    assert_eq!(stats.by_status.get("partial"), Some(&1));
    assert_eq!(stats.by_status.get("missing"), Some(&1));
}

// @req FR-COV-003
#[test]
fn annotation_stats_counts_by_type_and_orphans() {
    let annotations = vec![
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Impl),
        common::make_annotation("FR-COV-001", AnnotationType::Test),
        common::make_annotation("FR-COV-002", AnnotationType::Test),
    ];
    let stats = compute_annotation_stats(&annotations, 1);
    assert_eq!(stats.total, 4);
    assert_eq!(stats.impl_count, 2);
    assert_eq!(stats.test_count, 2);
    assert_eq!(stats.orphans, 1);
}

// @req FR-COV-003
#[test]
fn task_stats_counts_by_status_and_orphans() {
    let tasks = vec![
        common::make_task("TASK-001", "FR-COV-001", TaskStatus::Open),
        common::make_task("TASK-002", "FR-COV-001", TaskStatus::InProgress),
        common::make_task("TASK-003", "FR-COV-001", TaskStatus::Done),
        common::make_task("TASK-004", "FR-COV-002", TaskStatus::Done),
    ];
    let stats = compute_task_stats(&tasks, 2);
    assert_eq!(stats.total, 4);
    assert_eq!(stats.by_status.get("open"), Some(&1));
    assert_eq!(stats.by_status.get("in_progress"), Some(&1));
    assert_eq!(stats.by_status.get("done"), Some(&2));
    assert_eq!(stats.orphans, 2);
}

// @req FR-COV-003
#[test]
fn empty_inputs_produce_zero_stats() {
    let req_stats = compute_requirement_stats(&[], &[]);
    assert_eq!(req_stats.total, 0);
    assert!(req_stats.by_type.is_empty());
    assert!(req_stats.by_status.is_empty());

    let ann_stats = compute_annotation_stats(&[], 0);
    assert_eq!(ann_stats.total, 0);
    assert_eq!(ann_stats.impl_count, 0);
    assert_eq!(ann_stats.test_count, 0);
    assert_eq!(ann_stats.orphans, 0);

    let task_stats = compute_task_stats(&[], 0);
    assert_eq!(task_stats.total, 0);
    assert!(task_stats.by_status.is_empty());
    assert_eq!(task_stats.orphans, 0);
}
