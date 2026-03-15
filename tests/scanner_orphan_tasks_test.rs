use std::collections::HashSet;

use chrono::Utc;
use sdd_coverage::models::{Task, TaskStatus};
use sdd_coverage::scanner::find_orphan_tasks;

fn make_task(id: &str, requirement_id: &str) -> Task {
    Task {
        id: id.to_string(),
        requirement_id: requirement_id.to_string(),
        title: "Test task".to_string(),
        status: TaskStatus::Open,
        assignee: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// @req FR-SCAN-005
#[test]
fn detects_orphan_tasks() {
    let tasks = vec![
        make_task("TASK-001", "FR-EXISTS-001"),
        make_task("TASK-002", "FR-ORPHAN-001"),
        make_task("TASK-003", "FR-EXISTS-002"),
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
        make_task("TASK-001", "FR-EXISTS-001"),
        make_task("TASK-002", "FR-EXISTS-002"),
    ];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001", "FR-EXISTS-002"].into();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert!(orphans.is_empty());
}

// @req FR-SCAN-005
#[test]
fn all_orphans_when_none_match() {
    let tasks = vec![
        make_task("TASK-001", "FR-GONE-001"),
        make_task("TASK-002", "FR-GONE-002"),
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
    let tasks = vec![make_task("TASK-001", "FR-ANY-001")];
    let req_ids: HashSet<&str> = HashSet::new();

    let orphans = find_orphan_tasks(&tasks, &req_ids);
    assert_eq!(orphans.len(), 1);
}
