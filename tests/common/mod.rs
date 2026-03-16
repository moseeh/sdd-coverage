use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use chrono::Utc;
use tempfile::NamedTempFile;
use tokio::sync::RwLock;

use sdd_coverage::api::{AppState, ScanState, SharedState};
use sdd_coverage::config::ProjectConfig;
use sdd_coverage::models::{
    Annotation, AnnotationType, HealthStatus, Requirement, RequirementType, ScanResult, Task,
    TaskStatus,
};

// @req AR-DRY-001
pub fn make_dummy_config() -> ProjectConfig {
    ProjectConfig {
        requirements: PathBuf::from("r.yaml"),
        source: PathBuf::from("src"),
        tests: PathBuf::from("tests"),
    }
}

// @req AR-DRY-001
pub fn make_app_state(health_status: HealthStatus, scan_result: Option<ScanResult>) -> SharedState {
    let last_scan_at = if scan_result.is_some() {
        Some(Utc::now())
    } else {
        None
    };
    Arc::new(RwLock::new(AppState {
        scan_result,
        health_status,
        last_scan_at,
        scan_state: ScanState::Idle,
        scan_started_at: None,
        scan_completed_at: None,
        scan_duration_ms: None,
        scan_lock: Arc::new(AtomicBool::new(false)),
        config: make_dummy_config(),
    }))
}

// @req AR-DRY-001
pub fn make_requirement(id: &str, req_type: RequirementType) -> Requirement {
    Requirement {
        id: id.to_string(),
        req_type,
        title: "Test".to_string(),
        description: "Test".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// @req AR-DRY-001
pub fn make_annotation(req_id: &str, annotation_type: AnnotationType) -> Annotation {
    Annotation {
        file: "src/main.rs".to_string(),
        line: 1,
        req_id: req_id.to_string(),
        annotation_type,
        snippet: "// @req".to_string(),
    }
}

// @req AR-DRY-001
pub fn make_task(id: &str, requirement_id: &str, status: TaskStatus) -> Task {
    Task {
        id: id.to_string(),
        requirement_id: requirement_id.to_string(),
        title: "Test task".to_string(),
        status,
        assignee: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// @req AR-DRY-001
pub fn write_yaml_fixture(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file
}
