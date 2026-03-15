use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;
use walkdir::WalkDir;

use crate::models::Annotation;

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

// @req FR-SCAN-001
fn scan_directory(dir: &Path, base: &Path) -> Vec<Annotation> {
    let mut annotations = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() || !is_supported_extension(path) {
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let relative = path
            .strip_prefix(base)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let regex = req_regex();
        for (line_num, line) in content.lines().enumerate() {
            for cap in regex.captures_iter(line) {
                annotations.push(Annotation {
                    file: relative.clone(),
                    line: line_num + 1,
                    req_id: cap[1].to_string(),
                });
            }
        }
    }

    annotations
}

// @req FR-SCAN-001
pub fn scan_files(source: &Path, tests: &Path, base: &Path) -> Vec<Annotation> {
    let mut annotations = scan_directory(source, base);
    annotations.extend(scan_directory(tests, base));
    annotations
}
