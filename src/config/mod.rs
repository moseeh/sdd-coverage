use std::path::PathBuf;

use clap::Args;

// @req AR-CLI-001
#[derive(Args, Clone)]
pub struct ProjectConfig {
    /// Path to requirements.yaml
    #[arg(long)]
    pub requirements: PathBuf,
    /// Path to source directory
    #[arg(long)]
    pub source: PathBuf,
    /// Path to tests directory
    #[arg(long)]
    pub tests: PathBuf,
}
