use sdd_coverage::scanner::scan_files;

// @req FR-SCAN-006
#[test]
fn skips_unsupported_extensions() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    std::fs::write(source.join("data.json"), r#"{"@req": "FR-TEST-001"}"#).unwrap();
    std::fs::write(
        source.join("style.css"),
        concat!("/* @", "req FR-TEST-001 */"),
    )
    .unwrap();
    std::fs::write(source.join("readme.md"), concat!("// @", "req FR-TEST-001")).unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert!(annotations.is_empty());
}

// @req FR-SCAN-006
#[test]
fn skips_files_without_extensions() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    std::fs::write(source.join("Makefile"), concat!("# @", "req FR-TEST-001")).unwrap();
    std::fs::write(source.join("Dockerfile"), concat!("# @", "req FR-TEST-001")).unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert!(annotations.is_empty());
}

// @req FR-SCAN-006
#[test]
fn skips_binary_files() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    std::fs::write(source.join("image.png"), &[0x89, 0x50, 0x4E, 0x47]).unwrap();
    std::fs::write(source.join("binary.exe"), &[0x00, 0xFF, 0xFE]).unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert!(annotations.is_empty());
}

// @req FR-SCAN-006
#[test]
fn scans_only_supported_extensions() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    // Supported
    std::fs::write(
        source.join("main.rs"),
        concat!("// @", "req FR-TEST-001\nfn main() {}"),
    )
    .unwrap();
    std::fs::write(
        source.join("app.ts"),
        concat!("// @", "req FR-TEST-002\nconst x = 1;"),
    )
    .unwrap();
    std::fs::write(
        source.join("lib.py"),
        concat!("# @", "req FR-TEST-003\ndef f(): pass"),
    )
    .unwrap();
    // Unsupported
    std::fs::write(
        source.join("config.yaml"),
        concat!("# @", "req FR-TEST-004"),
    )
    .unwrap();
    std::fs::write(source.join("notes.txt"), concat!("// @", "req FR-TEST-005")).unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert_eq!(annotations.len(), 3);
}
