use chrono::Utc;
use sdd_coverage::coverage::{
    compute_annotation_stats, compute_requirement_stats, compute_task_stats,
};
use sdd_coverage::models::{
    Annotation, AnnotationType, Requirement, RequirementType, Task, TaskStatus,
};

fn make_requirement(id: &str, req_type: RequirementType) -> Requirement {
    Requirement {
        id: id.to_string(),
        req_type,
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

fn make_task(id: &str, req_id: &str, status: TaskStatus) -> Task {
    Task {
        id: id.to_string(),
        requirement_id: req_id.to_string(),
        title: "Test".to_string(),
        status,
        assignee: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// @req FR-COV-003
#[test]
fn requirement_stats_counts_by_type() {
    let reqs = vec![
        make_requirement("FR-COV-001", RequirementType::FR),
        make_requirement("FR-COV-002", RequirementType::FR),
        make_requirement("AR-CLI-001", RequirementType::AR),
    ];
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
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
        make_requirement("FR-COV-001", RequirementType::FR),
        make_requirement("FR-COV-002", RequirementType::FR),
        make_requirement("FR-COV-003", RequirementType::FR),
    ];
    let annotations = vec![
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
        make_annotation("FR-COV-002", AnnotationType::Impl),
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
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Impl),
        make_annotation("FR-COV-001", AnnotationType::Test),
        make_annotation("FR-COV-002", AnnotationType::Test),
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
        make_task("TASK-001", "FR-COV-001", TaskStatus::Open),
        make_task("TASK-002", "FR-COV-001", TaskStatus::InProgress),
        make_task("TASK-003", "FR-COV-001", TaskStatus::Done),
        make_task("TASK-004", "FR-COV-002", TaskStatus::Done),
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
