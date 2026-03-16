# SDD Coverage

Rust CLI + HTTP service that scans codebases for `@req` annotations and computes [Specification-Driven Development](https://blog.rezvov.com/specification-driven-development-four-pillars) traceability coverage metrics.

Part of the SDD toolchain. Consumed by a Next.js dashboard, Flutter app, and DevOps infrastructure.

## Sources of Truth

| Artifact | File |
|----------|------|
| What the service does | [`requirements.yaml`](requirements.yaml) |
| API contract | [`sdd-coverage-api.yaml`](sdd-coverage-api.yaml) |
| Work items | [`tasks.yaml`](tasks.yaml) |
| Engineering philosophy | [`sdd-four-pillars.md`](sdd-four-pillars.md) |
| Build & verify rules | [`CLAUDE.md`](CLAUDE.md) |

## Quick Start

```bash
cargo build --release
```

### Scan

```bash
./target/release/sdd-coverage scan \
  --requirements requirements.yaml \
  --source ./src \
  --tests ./tests \
  --strict
```

`--strict` exits with code 1 if any requirement is missing/partial or any orphans exist.

### Serve

```bash
./target/release/sdd-coverage serve \
  --requirements requirements.yaml \
  --source ./src \
  --tests ./tests \
  --port 4010
```

Performs an initial scan on startup. Endpoints conform to [`sdd-coverage-api.yaml`](sdd-coverage-api.yaml).

## Verification

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
./target/release/sdd-coverage scan \
  --requirements requirements.yaml \
  --source ./src \
  --tests ./tests \
  --strict
```

## Self-Hosting

The service scans its own codebase and passes `--strict` at 100% coverage with zero orphans.
