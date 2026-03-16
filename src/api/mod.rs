pub mod annotations;
pub mod healthcheck;
pub mod requirements;
pub mod scan;
pub mod stats;
pub mod tasks;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::config::ProjectConfig;
use crate::models::{HealthStatus, ScanResult};

// @req FR-API-007
#[derive(Debug, Clone, PartialEq)]
pub enum ScanState {
    Idle,
    Scanning,
    Completed,
    Failed,
}

// @req FR-API-001
pub struct AppState {
    pub scan_result: Option<ScanResult>,
    pub health_status: HealthStatus,
    // @req FR-API-002
    pub last_scan_at: Option<DateTime<Utc>>,
    // @req FR-API-007
    pub scan_state: ScanState,
    pub scan_started_at: Option<DateTime<Utc>>,
    pub scan_completed_at: Option<DateTime<Utc>>,
    pub scan_duration_ms: Option<i64>,
    pub scan_lock: Arc<AtomicBool>,
    pub config: ProjectConfig,
}

// @req FR-API-001
pub type SharedState = Arc<RwLock<AppState>>;
