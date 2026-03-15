use std::collections::HashSet;
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;
use walkdir::WalkDir;

use crate::models::{Annotation, AnnotationType, Task};

static REQ_PATTERN: OnceLock<Regex> = OnceLock::new();

// @req FR-SCAN-001
fn req_regex() -> &'static Regex {
    REQ_PATTERN.get_or_init(|| Regex::new(r"(?://+|#)\s*@req\s+([\w-]+)").unwrap())
}

// @req FR-SCAN-001
fn is_supported_extension(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("rs" | "ts" | "js" | "go" | "dart" | "py")
    )
}

// @req FR-SCAN-002
fn is_test_file(path: &Path, tests_dir: &Path) -> bool {
    if path.starts_with(tests_dir) {
        return true;
    }

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    stem.starts_with("test_") || stem.ends_with("_test") || name.contains(".test.")
}

// @req FR-SCAN-003
fn capture_snippet(lines: &[&str], line_index: usize) -> String {
    let end = (line_index + 2).min(lines.len());
    lines[line_index..end].join("\n")
}

// @req FR-SCAN-001
fn scan_directory(
    dir: &Path,
    base: &Path,
    tests_dir: &Path,
    warnings: &mut Vec<String>,
) -> Vec<Annotation> {
    let mut annotations = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || !is_supported_extension(path) {
            continue;
        }

        // @req FR-SCAN-003
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                warnings.push(format!(
                    "Permission error reading {}: {}",
                    path.display(),
                    e
                ));
                continue;
            }
        };

        let relative = path
            .strip_prefix(base)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        // @req FR-SCAN-002
        let annotation_type = if is_test_file(path, tests_dir) {
            AnnotationType::Test
        } else {
            AnnotationType::Impl
        };

        let lines: Vec<&str> = content.lines().collect();
        let regex = req_regex();
        for (line_num, line) in lines.iter().enumerate() {
            for cap in regex.captures_iter(line) {
                annotations.push(Annotation {
                    file: relative.clone(),
                    line: line_num + 1,
                    req_id: cap[1].to_string(),
                    annotation_type: annotation_type.clone(),
                    // @req FR-SCAN-003
                    snippet: capture_snippet(&lines, line_num),
                });
            }
        }
    }

    annotations
}

// @req FR-SCAN-001
pub fn scan_files(source: &Path, tests: &Path, base: &Path) -> (Vec<Annotation>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut annotations = scan_directory(source, base, tests, &mut warnings);
    annotations.extend(scan_directory(tests, base, tests, &mut warnings));
    (annotations, warnings)
}

// @req FR-SCAN-004
pub fn find_orphan_annotations<'a>(
    annotations: &'a [Annotation],
    requirement_ids: &HashSet<&str>,
) -> Vec<&'a Annotation> {
    annotations
        .iter()
        .filter(|a| !requirement_ids.contains(a.req_id.as_str()))
        .collect()
}

// @req FR-SCAN-005
pub fn find_orphan_tasks<'a>(tasks: &'a [Task], requirement_ids: &HashSet<&str>) -> Vec<&'a Task> {
    tasks
        .iter()
        .filter(|t| !requirement_ids.contains(t.requirement_id.as_str()))
        .collect()
}
