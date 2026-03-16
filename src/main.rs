use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use axum::Router;
use axum::routing::get;
use chrono::Utc;
use clap::Parser;
use tower_http::cors::CorsLayer;

use sdd_coverage::api::annotations::list_annotations;
use sdd_coverage::api::healthcheck::healthcheck;
use sdd_coverage::api::requirements::{get_requirement, list_requirements};
use sdd_coverage::api::scan::{get_scan_status, trigger_scan};
use sdd_coverage::api::stats::get_stats;
use sdd_coverage::api::tasks::list_tasks;
use sdd_coverage::api::{AppState, ScanState, SharedState};
use sdd_coverage::config::{Cli, Command};
use sdd_coverage::coverage::run_scan;
use sdd_coverage::error::fallback_handler;
use sdd_coverage::models::HealthStatus;

// @req FR-CLI-001
// @req FR-CLI-002
// @req AR-SELF-001
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan(args) => run_cli_scan(args),
        Command::Serve(args) => run_serve(args),
    }
}

// @req FR-CLI-001
fn run_cli_scan(args: sdd_coverage::config::ScanArgs) {
    let result = match run_scan(&args.config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Coverage: {:.1}%", result.coverage_percentage);
    println!(
        "Requirements: {} total ({} covered, {} partial, {} missing)",
        result.requirement_stats.total,
        result
            .requirement_stats
            .by_status
            .get("covered")
            .unwrap_or(&0),
        result
            .requirement_stats
            .by_status
            .get("partial")
            .unwrap_or(&0),
        result
            .requirement_stats
            .by_status
            .get("missing")
            .unwrap_or(&0),
    );
    println!(
        "Annotations: {} total ({} impl, {} test, {} orphans)",
        result.annotation_stats.total,
        result.annotation_stats.impl_count,
        result.annotation_stats.test_count,
        result.annotation_stats.orphans,
    );
    println!(
        "Tasks: {} total ({} orphans)",
        result.task_stats.total, result.task_stats.orphans,
    );

    for warning in &result.warnings {
        eprintln!("Warning: {}", warning);
    }

    if args.strict {
        let has_missing = result
            .requirement_stats
            .by_status
            .get("missing")
            .unwrap_or(&0)
            > &0;
        let has_partial = result
            .requirement_stats
            .by_status
            .get("partial")
            .unwrap_or(&0)
            > &0;
        let has_orphan_annotations = result.annotation_stats.orphans > 0;
        let has_orphan_tasks = result.task_stats.orphans > 0;

        if has_missing || has_partial || has_orphan_annotations || has_orphan_tasks {
            eprintln!("Strict mode: failing due to incomplete coverage or orphans");
            std::process::exit(1);
        }
    }
}

// @req FR-CLI-002
fn run_serve(args: sdd_coverage::config::ServeArgs) {
    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    rt.block_on(async move {
        let scan_result = match run_scan(&args.config) {
            Ok(r) => {
                println!(
                    "Initial scan completed: {:.1}% coverage",
                    r.coverage_percentage
                );
                Some(r)
            }
            Err(e) => {
                eprintln!("Initial scan failed: {}", e);
                None
            }
        };

        let health_status = if scan_result.is_some() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };

        let state: SharedState = Arc::new(tokio::sync::RwLock::new(AppState {
            scan_result,
            health_status,
            last_scan_at: Some(Utc::now()),
            scan_state: ScanState::Completed,
            scan_started_at: Some(Utc::now()),
            scan_completed_at: Some(Utc::now()),
            scan_duration_ms: Some(0),
            scan_lock: Arc::new(AtomicBool::new(false)),
            config: args.config,
        }));

        let app = Router::new()
            .route("/healthcheck", get(healthcheck))
            .route("/stats", get(get_stats))
            .route("/requirements", get(list_requirements))
            .route("/requirements/{id}", get(get_requirement))
            .route("/annotations", get(list_annotations))
            .route("/tasks", get(list_tasks))
            .route("/scan", get(get_scan_status).post(trigger_scan))
            .fallback(fallback_handler)
            .layer(CorsLayer::permissive())
            .with_state(state);

        let addr = format!("0.0.0.0:{}", args.port);
        println!("Listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("failed to bind");
        axum::serve(listener, app).await.expect("server error");
    });
}
