use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

// @req AR-CLI-001
#[derive(Args, Clone)]
pub struct ProjectConfig {
    #[arg(long)]
    pub requirements: PathBuf,
    #[arg(long)]
    pub source: PathBuf,
    #[arg(long)]
    pub tests: PathBuf,
}

// @req FR-CLI-001
#[derive(Parser)]
#[command(name = "sdd-coverage", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// @req FR-CLI-001
// @req FR-CLI-002
#[derive(Subcommand)]
pub enum Command {
    Scan(ScanArgs),
    Serve(ServeArgs),
}

// @req FR-CLI-002
#[derive(Args)]
pub struct ServeArgs {
    #[command(flatten)]
    pub config: ProjectConfig,
    #[arg(long, default_value_t = 4010)]
    pub port: u16,
}

// @req FR-CLI-001
#[derive(Args)]
pub struct ScanArgs {
    #[command(flatten)]
    pub config: ProjectConfig,
    #[arg(long, default_value_t = false)]
    pub strict: bool,
}
