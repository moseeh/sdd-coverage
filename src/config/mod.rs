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
#[derive(Subcommand)]
pub enum Command {
    Scan(ScanArgs),
}

// @req FR-CLI-001
#[derive(Args)]
pub struct ScanArgs {
    #[command(flatten)]
    pub config: ProjectConfig,
    #[arg(long, default_value_t = false)]
    pub strict: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    // @req FR-CLI-001
    #[test]
    fn parse_scan_with_required_flags() {
        let cli = Cli::try_parse_from([
            "sdd-coverage",
            "scan",
            "--requirements",
            "req.yaml",
            "--source",
            "./src",
            "--tests",
            "./tests",
        ])
        .unwrap();

        match cli.command {
            Command::Scan(args) => {
                assert_eq!(args.config.requirements, PathBuf::from("req.yaml"));
                assert_eq!(args.config.source, PathBuf::from("./src"));
                assert_eq!(args.config.tests, PathBuf::from("./tests"));
                assert!(!args.strict);
            }
        }
    }

    // @req FR-CLI-001
    #[test]
    fn parse_scan_with_strict() {
        let cli = Cli::try_parse_from([
            "sdd-coverage",
            "scan",
            "--requirements",
            "req.yaml",
            "--source",
            "./src",
            "--tests",
            "./tests",
            "--strict",
        ])
        .unwrap();

        match cli.command {
            Command::Scan(args) => {
                assert!(args.strict);
            }
        }
    }

    // @req FR-CLI-001
    #[test]
    fn scan_fails_without_required_flags() {
        let result = Cli::try_parse_from(["sdd-coverage", "scan"]);
        assert!(result.is_err());
    }
}
