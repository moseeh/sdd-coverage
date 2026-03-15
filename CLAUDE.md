# SDD Coverage Service

## What
Rust CLI + HTTP service that scans codebases for @req annotations
and computes SDD traceability coverage metrics.

## Read These First (before writing any code)
- PROBLEM.md — full task description and evaluation criteria
- requirements.yaml — source of truth for what this service does
- tasks.yaml — work items linked to requirements, update status on each commit
- sdd-coverage-api.yaml — API contract all endpoints must conform to (immutable)
- sdd-four-pillars.md — engineering philosophy this project follows

## How to Verify Changes
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test
- cargo build --release
./target/release/sdd-coverage scan --requirements requirements.yaml --source ./src --tests ./tests --strict

## Rules
- Read requirements.yaml before implementing anything
- Every function MUST have a // @req FR-XXX-NNN or // @req AR-XXX-NNN annotation
- Every implementation commit MUST reference a requirement ID
- One task per commit — each task maps to one requirement
- Add dependencies incrementally — only what the current task needs
- Update tasks.yaml status and updatedAt (EAT timezone) in each commit
- Tests MUST accompany each task, annotated with // @req for the requirement
- Scanner logic MUST be shared between CLI and HTTP modes
- No panics under any input
- Dependencies in Cargo.toml must be minimal and justified
- sdd-coverage-api.yaml MUST NOT be modified

## Commit Convention
chore: description                            <- setup, config
feat(module): description [REQ-ID]       <- implementation
test(module): description [REQ-ID]       <- tests
fix(module): description [REQ-ID]        <- bug fixes

## Project Structure
src/                 implementation (each module in its own directory)
  config/mod.rs      shared ProjectConfig, Cli, ScanArgs, ServeArgs
  models/mod.rs      data types (added per-task as needed)
  error/mod.rs       AppError enum (added per-task as needed)
  parser/mod.rs      YAML parsing (added per-task as needed)
  scanner/mod.rs     file walking + @req extraction (added per-task as needed)
  coverage/mod.rs    coverage computation + run_scan (added per-task as needed)
  api/               HTTP endpoints (added per-task as needed)
tests/               integration tests
fixtures/            sample projects for integration tests
