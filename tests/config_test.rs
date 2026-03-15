use std::path::PathBuf;

use sdd_coverage::config::{Cli, Command};

use clap::Parser;

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
        _ => panic!("expected Scan"),
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
        _ => panic!("expected Scan"),
    }
}

// @req FR-CLI-001
#[test]
fn scan_fails_without_required_flags() {
    let result = Cli::try_parse_from(["sdd-coverage", "scan"]);
    assert!(result.is_err());
}

// @req FR-CLI-002
#[test]
fn parse_serve_with_required_flags() {
    let cli = Cli::try_parse_from([
        "sdd-coverage",
        "serve",
        "--requirements",
        "req.yaml",
        "--source",
        "./src",
        "--tests",
        "./tests",
    ])
    .unwrap();

    match cli.command {
        Command::Serve(args) => {
            assert_eq!(args.config.requirements, PathBuf::from("req.yaml"));
            assert_eq!(args.config.source, PathBuf::from("./src"));
            assert_eq!(args.config.tests, PathBuf::from("./tests"));
            assert_eq!(args.port, 4010);
        }
        _ => panic!("expected Serve"),
    }
}

// @req FR-CLI-002
#[test]
fn parse_serve_with_custom_port() {
    let cli = Cli::try_parse_from([
        "sdd-coverage",
        "serve",
        "--requirements",
        "req.yaml",
        "--source",
        "./src",
        "--tests",
        "./tests",
        "--port",
        "8080",
    ])
    .unwrap();

    match cli.command {
        Command::Serve(args) => {
            assert_eq!(args.port, 8080);
        }
        _ => panic!("expected Serve"),
    }
}

// @req FR-CLI-002
#[test]
fn serve_fails_without_required_flags() {
    let result = Cli::try_parse_from(["sdd-coverage", "serve"]);
    assert!(result.is_err());
}
