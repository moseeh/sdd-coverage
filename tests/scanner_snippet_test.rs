use std::path::Path;

use sdd_coverage::scanner::scan_files;

// @req FR-SCAN-003
#[test]
fn snippet_includes_annotation_and_next_line() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let (annotations, _) = scan_files(&source, &tests, base);

    let main_rs: Vec<_> = annotations
        .iter()
        .filter(|a| a.file == "src/main.rs")
        .collect();
    assert_eq!(main_rs[0].snippet, "// @req FR-TEST-001\nfn main() {");
}

// @req FR-SCAN-003
#[test]
fn snippet_at_last_line_contains_only_annotation() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    std::fs::write(source.join("end.rs"), "// @req FR-TEST-001").unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].snippet, "// @req FR-TEST-001");
}

// @req FR-SCAN-003
#[test]
fn snippet_captures_python_context() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let (annotations, _) = scan_files(&source, &tests, base);

    let py: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.ends_with(".py"))
        .collect();
    assert_eq!(py.len(), 1);
    assert!(py[0].snippet.contains("# @req FR-TEST-003"));
    assert!(py[0].snippet.contains("def compute():"));
}

// @req FR-SCAN-003
#[test]
fn permission_error_produces_warning_and_continues() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    // Create a readable file
    std::fs::write(
        source.join("good.rs"),
        "// @req FR-TEST-001\nfn good() {}\n",
    )
    .unwrap();

    // Create an unreadable file
    let bad_path = source.join("bad.rs");
    std::fs::write(&bad_path, "// @req FR-TEST-002\nfn bad() {}\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bad_path, std::fs::Permissions::from_mode(0o000)).unwrap();
    }

    let (annotations, warnings) = scan_files(&source, &tests, dir.path());

    // Good file should still be scanned
    assert!(
        annotations.iter().any(|a| a.req_id == "FR-TEST-001"),
        "should have scanned the readable file"
    );

    // On Unix, bad file should produce a warning
    #[cfg(unix)]
    {
        assert!(
            !warnings.is_empty(),
            "expected a warning for unreadable file"
        );
        assert!(warnings[0].contains("Permission error"));

        // Restore permissions for cleanup
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&bad_path, std::fs::Permissions::from_mode(0o644)).unwrap();
    }

    // Suppress unused variable warning on non-Unix
    #[cfg(not(unix))]
    let _ = warnings;
}
