pub mod annotations;
pub mod healthcheck;
pub mod requirements;
pub mod stats;
pub mod tasks;

use std::sync::Arc;

use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::models::{HealthStatus, ScanResult};

// @req FR-API-001
pub struct AppState {
    pub scan_result: Option<ScanResult>,
    pub health_status: HealthStatus,
    // @req FR-API-002
    pub last_scan_at: Option<DateTime<Utc>>,
}

// @req FR-API-001
pub type SharedState = Arc<RwLock<AppState>>;
