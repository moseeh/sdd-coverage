# Development Process

## 1. Tools Used

| Tool | Purpose |
|------|---------|
| Claude Code (CLI) | Primary development tool. All code generation, refactoring, testing, and git operations performed through conversational AI |
| Claude Opus 4.6 | Model used throughout all sessions |
| rust-analyzer LSP | Installed as Claude Code plugin during Session 1 for Rust language support |
| cargo | Build, test, lint, format — deterministic verification pipeline |
| curl + jq | Manual endpoint verification of the running HTTP server |

No IDE was used for manual code editing. All code was written, edited, and committed through Claude Code.

## 2. Conversation Log

Development occurred in a single continuous Claude Code conversation with 5 context compactions (automatic context window resets that preserve a summary of prior work).

### Session 1 — Project Setup and Planning
- **Start:** 2026-03-15T19:54:20Z
- **End:** 2026-03-15T22:28:17Z (context compaction)
- **Topic:** Project understanding, planning, initial skeleton
- **Developer asked for:** Read all project files, produce an implementation plan, begin coding
- **Accepted:** Implementation plan with 8 phases, 28 requirements, module architecture
- **Rejected/Corrected:**
  - AI tried to commit with generic message — developer required checking diff against origin first
  - AI added `Co-Authored-By` to commit messages — developer rejected, required single-line commits only
  - AI added obvious comments — developer said "avoid obvious comments"
  - AI skipped tests — developer asked "no tests, methods to accompany these?"
  - AI imported `axum` in error module prematurely — developer caught unnecessary dependency
  - AI annotated code with multiple requirement IDs in one function — developer enforced one-requirement-per-commit discipline
  - AI used hardcoded version 3.0.0 from API spec in Cargo.toml — developer questioned it
  - AI added all dependencies upfront — developer enforced incremental dependency addition per task
  - AI used UTC timestamps — developer required EAT (East African Time) for tasks.yaml updatedAt
  - AI used `#[default]` attribute on enum variant — developer questioned why it needed special treatment

### Session 2 — Parsing and Scanning
- **Start:** 2026-03-15T22:28:17Z
- **End:** 2026-03-15T23:43:42Z (context compaction)
- **Topic:** YAML parsers, file scanner, test structure
- **Developer asked for:** Implement FR-PARSE-001 through FR-PARSE-004, FR-SCAN-001 through FR-SCAN-006
- **Accepted:** Parser with validation, scanner with multi-language support, orphan detection
- **Rejected/Corrected:**
  - AI tried to put tests inline (unit tests in source files) — developer asked about Rust conventions for integration tests, moved to `tests/` directory
  - AR-CLI-001 had no tests — developer enforced "each requirement should have tests"
  - AI tried to commit multiple requirement changes in one commit — developer asked "we can't commit them all with the same commit because of different IDs.. what do you suggest"
  - Developer noticed no ID format validation existed — proposed new requirement FR-PARSE-005 for `TYPE-DOMAIN-NNN` pattern validation

### Session 3 — Coverage Engine and API
- **Start:** 2026-03-15T23:43:42Z
- **End:** 2026-03-16T12:03:09Z (context compaction, includes overnight break)
- **Topic:** Coverage computation, all REST API endpoints, error handling
- **Developer asked for:** FR-COV-001 through FR-COV-003, FR-API-001 through FR-API-008, FR-ERR-001
- **Accepted:** Coverage engine, all 8 API endpoints, error handling
- **Rejected/Corrected:**
  - AI used wrong commit message convention after context compaction — developer corrected: "that is not the commit message convention we have been using"

### Session 4 — Self-Hosting and Traceability Fixes
- **Start:** 2026-03-16T12:03:09Z
- **End:** 2026-03-16T12:56:19Z (context compaction)
- **Topic:** Wire up main.rs, self-scan, traceability violation fixes
- **Developer asked for:** Complete AR-SELF-001, fix traceability violations
- **Accepted:** Self-scan test, `concat!` macro fix for fixture annotations
- **Rejected/Corrected:**
  - AI staged changes spanning multiple requirements in one commit — developer caught it: "several requirements have been modified here and for traceability we cannot commit them all under one requirement id"
  - AI left test helper functions without `@req` annotations — developer caught it: "also i am seeing functions/tests without any annotations previously staged but that breaks the traceability"
  - Developer identified DRY violation in duplicated test helpers across 10+ files: "so essentially we are violating DRY with these"
  - Developer chose to finish pending commits before creating the DRY requirement: "no lets finish with the three commits first then we create a requirement and task for it then refactor"

### Session 5 — DRY Refactor
- **Start:** 2026-03-16T12:56:19Z
- **End:** 2026-03-16T13:34:47Z
- **Topic:** Extract shared test helpers, final verification, README
- **Developer asked for:** Complete AR-DRY-001, verify serve endpoint, write README
- **Accepted:** Common test module, endpoint verification results
- **Rejected/Corrected:**
  - AI tried to stage all test files individually by requirement — developer asked "would it be logical to combine several requirement ids into one commit cause they do the same modification"
  - AI included `Co-Authored-By` in commit again — developer corrected: "that is not the commit convention we have been using"
  - AI wrote README that duplicated API endpoints, project structure, and dependencies — developer caught DRY violation: "you didn't go through the 4 pillars file?" — README was rewritten to reference sources of truth instead
  - Developer caught duplicate `updatedAt` field in tasks.yaml when running the serve command — triggered a fix commit
  - Developer insisted on manual endpoint verification: "just give me the command to run it and then in one terminal we run it .. in another we curl it"

## 3. Timeline

| Time (EAT, UTC+3) | Duration | Activity |
|---|---|---|
| 2026-03-15 22:54 | 25 min | Project reading, planning, architecture design |
| 2026-03-15 23:19 | 5 min | Initial project scaffolding (4 setup commits) |
| 2026-03-15 23:30 | 30 min | Requirements and tasks authoring (3 commits) |
| 2026-03-16 00:04 | 65 min | CLI skeleton: AR-CLI-001, FR-CLI-001, FR-CLI-002 (5 commits) |
| 2026-03-16 01:51 | 50 min | Parsers: FR-PARSE-001 through FR-PARSE-005 (7 commits) |
| 2026-03-16 02:16 | 20 min | Scanner: FR-SCAN-001 through FR-SCAN-006 (7 commits) |
| *Overnight break* | ~11 hrs | — |
| 2026-03-16 13:58 | 15 min | Coverage: FR-COV-001 through FR-COV-003 (3 commits) |
| 2026-03-16 14:15 | 40 min | API: FR-API-001 through FR-API-008, FR-ERR-001 (9 commits) |
| 2026-03-16 15:38 | 5 min | Self-hosting fixes: FR-API-007, FR-CLI-001/002, AR-SELF-001 (3 commits) |
| 2026-03-16 15:42 | 40 min | DRY refactor: AR-DRY-001 (2 commits) |
| 2026-03-16 16:22 | 15 min | Serve verification, tasks.yaml fix, README (2 commits) |
| **Total active time** | **~5 hrs** | **45 commits, 28 requirements, 146 tests** |

## 4. Key Decisions

### Architecture: Single `run_scan()` entry point
Scanner logic shared between CLI and HTTP via `run_scan()` in `coverage/mod.rs`. Alternative was separate scan implementations for each mode. Chosen to satisfy DRY — one function, two callers.

### Module layout: Directory-based modules (`src/parser/mod.rs`)
Each module in its own directory. Alternative was flat files (`src/parser.rs`). Directory layout chosen for consistency with `src/api/` which has multiple files.

### Async scan: `tokio::task::spawn` with `AtomicBool` lock
POST /scan spawns an async task and returns 202 immediately. Concurrent scan rejection uses `AtomicBool::compare_exchange` — lighter than `Mutex`. Alternative was `tokio::sync::Mutex` with `try_lock()`.

### Test helpers: `tests/common/mod.rs` directory module
Rust treats files in `tests/` as separate test crates. A file `tests/common.rs` would be compiled as its own test binary. The `tests/common/mod.rs` directory pattern makes it a reusable module without becoming a test binary.

### Fixture annotation escaping: `concat!("// @", "req FR-TEST-001")`
Test fixture strings containing `// @req` were detected as real annotations by the self-scanner, producing 19 orphans. The `concat!` macro splits the token at compile time so the scanner regex never matches.

### Coverage classification: Test-only = missing
A requirement with only test annotations (no impl) is classified as `missing`, not `partial`. Rationale: no implementation exists regardless of test coverage.

### Commit convention: Single-line, no Co-Authored-By
Developer established `feat(module): description [REQ-ID]` as the commit format. No body, no trailer. AI repeatedly added `Co-Authored-By` and was corrected each time.

### Incremental dependencies
Dependencies added only when the task requiring them is implemented. AI initially tried to add all dependencies in phase 1 — developer enforced incremental addition.

## 5. What the Developer Controlled

### Requirement Authoring
Developer reviewed and shaped all 28 requirements. Key interventions:
- Questioned `#[default]` on `CoverageStatus::Missing` — asked AI to justify
- Proposed FR-PARSE-005 (ID format validation) — AI hadn't considered it
- Created AR-DRY-001 after identifying duplicated test helpers

### Commit Discipline
Developer enforced every commit boundary:
- Rejected multi-requirement commits repeatedly
- Defined commit message format and corrected AI 3 times when it deviated
- Enforced EAT timestamps in tasks.yaml (AI used UTC initially)
- Required `@req` annotations on every function including test helpers

### Traceability Verification
Developer caught traceability violations the AI missed:
- Unannotated test helper functions (`make_state`, `make_app`)
- Mixed requirement changes in single staged files
- Orphan annotations from fixture strings

### Manual Endpoint Testing
Developer ran the HTTP server and verified all endpoints via curl rather than trusting integration tests alone.

### README Review
Developer rejected the first README draft for violating DRY (duplicated API endpoints, project structure, dependencies). Required rewrite referencing sources of truth.

### Files the developer directly influenced through corrections:
- `CLAUDE.md` — commit convention, project structure, verification pipeline
- `requirements.yaml` — all 28 requirement definitions
- `tasks.yaml` — timestamps, status tracking
- `src/config/mod.rs` — default port from API spec
- `tests/common/mod.rs` — design driven by developer's DRY observation
- `README.md` — rejected first version, shaped final version

## 6. Course Corrections

### 1. Commit message format (3 occurrences)
- **Issue:** AI added `Co-Authored-By` trailer and multi-line commit bodies
- **How caught:** Developer read the commit output
- **Resolution:** Developer established single-line format, corrected AI each time it reverted after context compaction

### 2. Multi-requirement commits (3 occurrences)
- **Issue:** AI staged changes touching multiple requirements in one commit
- **How caught:** Developer reviewed staged files before committing
- **Resolution:** Decomposed into separate commits per requirement. In Session 4, split one commit into 3 traceable commits (FR-API-007, FR-CLI-001/002, AR-SELF-001)

### 3. Missing test annotations
- **Issue:** Test helper functions (`make_state`, `make_app`, `make_scan_result`) had no `@req` annotations
- **How caught:** Developer inspected staged diffs
- **Resolution:** Added annotations to all helper functions, created AR-DRY-001 for systematic cleanup

### 4. Premature dependency loading
- **Issue:** AI added all Cargo.toml dependencies (axum, tokio, walkdir, etc.) in phase 1
- **How caught:** Developer asked "axum is used in errors?" when seeing `axum` imported in `error/mod.rs`
- **Resolution:** Dependencies added incrementally per task

### 5. Duplicated test helpers (DRY violation)
- **Issue:** `make_state()`, `make_app()`, `write_yaml()` duplicated across 10+ test files
- **How caught:** Developer identified the pattern during traceability review
- **Resolution:** Created AR-DRY-001 requirement, extracted shared helpers into `tests/common/mod.rs`

### 6. Orphan annotations from fixture strings
- **Issue:** Self-scan reported 19 orphan annotations — test fixture strings like `"// @req FR-TEST-001"` matched the scanner regex
- **How caught:** Running `--strict` self-scan failed
- **Resolution:** Used `concat!("// @", "req FR-TEST-001")` to break the token

### 7. README duplicating sources of truth
- **Issue:** First README draft listed API endpoints, project structure, dependencies — all of which exist authoritatively in other files
- **How caught:** Developer asked "you didn't go through the 4 pillars file?" — referencing the DRY pillar's explicit warning against copying endpoint descriptions into READMEs
- **Resolution:** Rewrote README to reference `sdd-coverage-api.yaml`, `requirements.yaml`, etc. instead of duplicating their content

### 8. Duplicate `updatedAt` in tasks.yaml
- **Issue:** TASK-028 had two `updatedAt` fields, causing a parse error on server startup
- **How caught:** Developer ran the serve command and saw the error output
- **Resolution:** Removed the duplicate field

## 7. Self-Assessment

### Traceability — Strong
- Every function annotated with `// @req`
- Every commit references a requirement ID
- Every test traces to a specification
- Self-scan passes `--strict` at 100% coverage with 0 orphans
- Bidirectional: requirement → code → test → commit (top-down) and code → requirement (bottom-up via scanner)

**Improvement needed:** Some `@req` annotations on test helpers are arguably forced. `AR-DRY-001` on `AppState` exists only to satisfy the scanner's impl/test classification, not because `AppState` implements DRY. A more nuanced scanner could classify `tests/common/mod.rs` as impl rather than test.

### DRY — Strong
- `requirements.yaml` is the single source of truth for what the service does
- `sdd-coverage-api.yaml` is the single source of truth for the API contract (never modified)
- `tasks.yaml` is the single source of truth for work items
- Test helpers extracted into `tests/common/mod.rs`
- README references sources of truth instead of duplicating them

**Improvement needed:** Route-specific `make_app()` and `make_scan_result()` still exist locally in each API test file. These are similar in structure but differ in routes/data, so full extraction would require a builder pattern that may add more complexity than it removes.

### Deterministic Enforcement — Strong
- Full verification pipeline: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build --release`, self-scan `--strict`
- 146 tests cover parsing, scanning, coverage computation, all API endpoints, error handling, CLI argument parsing
- Self-hosting: the service validates its own traceability

**Improvement needed:** No CI pipeline configured. The verification commands exist but must be run manually. A GitHub Actions workflow would make enforcement truly deterministic on every push.

### Parsimony — Adequate
- Requirements use directive vocabulary (MUST/SHOULD)
- Commit messages are compressed: `feat(module): description [REQ-ID]`
- README is minimal — no redundant content
- No dead code, no unused dependencies

**Improvement needed:** Some test files are verbose — each API test constructs full `ScanResult` structs inline with multiple `HashMap::new()` calls. A builder pattern or fixture factory could reduce this without sacrificing clarity. The `sdd-four-pillars.md` file is included in the repository but is third-party reference material, not a project artifact — could be referenced by URL instead.
