mod common;

use std::collections::HashSet;

use sdd_coverage::models::{Task, TaskStatus};
use sdd_coverage::scanner::find_orphan_tasks;

// @req FR-SCAN-005
#[test]
fn detects_orphan_tasks() {
    let tasks = vec![
        common::make_task("TASK-001", "FR-EXISTS-001", TaskStatus::Open),
        common::make_task("TASK-002", "FR-ORPHAN-001", TaskStatus::Open),
        common::make_task("TASK-003", "FR-EXISTS-002", TaskStatus::Open),
    ];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001", "FR-EXISTS-002"].into();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0].id, "TASK-002");
}

// @req FR-SCAN-005
#[test]
fn no_orphans_when_all_match() {
    let tasks = vec![
        common::make_task("TASK-001", "FR-EXISTS-001", TaskStatus::Open),
        common::make_task("TASK-002", "FR-EXISTS-002", TaskStatus::Open),
    ];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001", "FR-EXISTS-002"].into();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert!(orphans.is_empty());
}

// @req FR-SCAN-005
#[test]
fn all_orphans_when_none_match() {
    let tasks = vec![
        common::make_task("TASK-001", "FR-GONE-001", TaskStatus::Open),
        common::make_task("TASK-002", "FR-GONE-002", TaskStatus::Open),
    ];
    let req_ids: HashSet<&str> = ["FR-OTHER-001"].into();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert_eq!(orphans.len(), 2);
}

// @req FR-SCAN-005
#[test]
fn empty_tasks_returns_empty() {
    let tasks: Vec<Task> = vec![];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001"].into();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert!(orphans.is_empty());
}

// @req FR-SCAN-005
#[test]
fn empty_requirements_makes_all_orphans() {
    let tasks = vec![common::make_task(
        "TASK-001",
        "FR-ANY-001",
        TaskStatus::Open,
    )];
    let req_ids: HashSet<&str> = HashSet::new();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert_eq!(orphans.len(), 1);
}
