use std::path::PathBuf;

use sdd_coverage::config::ProjectConfig;
use sdd_coverage::coverage::run_scan;

// @req FR-COV-003
#[test]
fn run_scan_produces_complete_result() {
    let config = ProjectConfig {
        requirements: PathBuf::from("fixtures/scan_project/requirements.yaml"),
        source: PathBuf::from("fixtures/scan_project/src"),
        tests: PathBuf::from("fixtures/scan_project/tests"),
    };
    let result = run_scan(&config).unwrap();

    assert_eq!(result.requirements.len(), 3);
    assert_eq!(result.tasks.len(), 2);
    assert!(!result.annotations.is_empty());

    // FR-TEST-001 has impl (main.rs, app.ts) + test (test_main.rs) => covered
    // FR-TEST-002 has impl (main.rs) only => partial
    // AR-TEST-001 has no annotations => missing
    assert_eq!(result.requirement_stats.by_status.get("covered"), Some(&1));
    assert_eq!(result.requirement_stats.by_status.get("partial"), Some(&1));
    assert_eq!(result.requirement_stats.by_status.get("missing"), Some(&1));

    assert_eq!(result.requirement_stats.by_type.get("FR"), Some(&2));
    assert_eq!(result.requirement_stats.by_type.get("AR"), Some(&1));

    assert!(result.annotation_stats.impl_count > 0);
    assert!(result.annotation_stats.test_count > 0);
    // FR-TEST-003 in widget.dart and lib.py are orphans (not in requirements)
    assert_eq!(result.annotation_stats.orphans, 2);

    assert_eq!(result.task_stats.by_status.get("done"), Some(&1));
    assert_eq!(result.task_stats.by_status.get("open"), Some(&1));
    assert_eq!(result.task_stats.orphans, 0);

    // 1 covered out of 3 total
    assert!((result.coverage_percentage - 100.0 / 3.0).abs() < 0.01);
}

// @req FR-COV-003
#[test]
fn run_scan_detects_orphan_annotations() {
    // The scan_project src files contain @req FR-TEST-001 and FR-TEST-002
    // but also app.ts has FR-TEST-001 which is valid
    // lib.py and others may have annotations not in requirements
    let config = ProjectConfig {
        requirements: PathBuf::from("fixtures/scan_project/requirements.yaml"),
        source: PathBuf::from("fixtures/scan_project/src"),
        tests: PathBuf::from("fixtures/scan_project/tests"),
    };
    let result = run_scan(&config).unwrap();
    // FR-TEST-003 in widget.dart and lib.py are orphans (not in requirements)
    assert_eq!(result.orphan_annotations.len(), 2);
    assert!(
        result
            .orphan_annotations
            .iter()
            .all(|a| a.req_id == "FR-TEST-003")
    );
    assert_eq!(result.orphan_tasks.len(), 0);
}
