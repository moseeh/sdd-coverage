# SDD Coverage Service

## What
Rust CLI + HTTP service that scans codebases for @req annotations
and computes SDD traceability coverage metrics.

## Read These First (before writing any code)
- PROBLEM.md — full task description and evaluation criteria
- requirements.yaml — source of truth for what this service does
- sdd-coverage-api.yaml — API contract all endpoints must conform to
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
- Scanner logic MUST be shared between CLI and HTTP modes
- No panics under any input
- Dependencies in Cargo.toml must be minimal and justified

## Commit Convention
chore: description                            <- setup, config
feat(module): description [REQ-ID]       <- implementation
test(module): description [REQ-ID]       <- tests
fix(module): description [REQ-ID]        <- bug fixes

## Project Structure
src/          implementation
tests/        integration tests
fixtures/     sample projects for integration tests