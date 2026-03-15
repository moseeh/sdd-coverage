use std::path::Path;

use sdd_coverage::scanner::scan_files;

// @req FR-SCAN-001
#[test]
fn scans_rust_source_files() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let rust_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".rs"))
        .collect();
    assert!(
        rust_annotations.len() >= 2,
        "expected at least 2 Rust annotations, got {}",
        rust_annotations.len()
    );
}

// @req FR-SCAN-001
#[test]
fn scans_typescript_files() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let ts_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".ts"))
        .collect();
    assert_eq!(ts_annotations.len(), 1);
    assert_eq!(ts_annotations[0].req_id, "FR-TEST-001");
}

// @req FR-SCAN-001
#[test]
fn scans_python_files_with_hash_comments() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let py_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".py"))
        .collect();
    assert_eq!(py_annotations.len(), 1);
    assert_eq!(py_annotations[0].req_id, "FR-TEST-003");
}

// @req FR-SCAN-001
#[test]
fn scans_go_files() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let go_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".go"))
        .collect();
    assert_eq!(go_annotations.len(), 1);
    assert_eq!(go_annotations[0].req_id, "FR-TEST-002");
}

// @req FR-SCAN-001
#[test]
fn scans_dart_files() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let dart_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".dart"))
        .collect();
    assert_eq!(dart_annotations.len(), 1);
    assert_eq!(dart_annotations[0].req_id, "FR-TEST-003");
}

// @req FR-SCAN-001
#[test]
fn skips_unsupported_extensions() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let json_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".json"))
        .collect();
    assert!(json_annotations.is_empty());
}

// @req FR-SCAN-001
#[test]
fn captures_correct_line_numbers() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let main_rs: Vec<_> = annotations
        .iter()
        .filter(|a| a.file == "src/main.rs")
        .collect();
    assert_eq!(main_rs.len(), 2);
    assert_eq!(main_rs[0].line, 1);
    assert_eq!(main_rs[0].req_id, "FR-TEST-001");
    assert_eq!(main_rs[1].line, 6);
    assert_eq!(main_rs[1].req_id, "FR-TEST-002");
}

// @req FR-SCAN-001
#[test]
fn scans_both_source_and_test_paths() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    let test_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.contains("tests/"))
        .collect();
    assert!(!test_annotations.is_empty());
}

// @req FR-SCAN-001
#[test]
fn returns_relative_paths() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let annotations = scan_files(&source, &tests, base);

    for annotation in &annotations {
        assert!(
            !annotation.file.starts_with("fixtures/scan_project"),
            "path should be relative to base: {}",
            annotation.file
        );
    }
}

// @req FR-SCAN-001
#[test]
fn handles_empty_directory() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    let annotations = scan_files(&source, &tests, dir.path());
    assert!(annotations.is_empty());
}
