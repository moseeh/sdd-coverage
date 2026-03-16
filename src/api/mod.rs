pub mod healthcheck;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::{HealthStatus, ScanResult};

// @req FR-API-001
pub struct AppState {
    pub scan_result: Option<ScanResult>,
    pub health_status: HealthStatus,
}

// @req FR-API-001
pub type SharedState = Arc<RwLock<AppState>>;
