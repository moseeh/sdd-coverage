use std::path::Path;

use sdd_coverage::models::AnnotationType;
use sdd_coverage::scanner::scan_files;

// @req FR-SCAN-002
#[test]
fn files_under_tests_path_classified_as_test() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let (annotations, _) = scan_files(&source, &tests, base);

    let test_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.starts_with("tests/"))
        .collect();
    assert!(!test_annotations.is_empty());
    for a in &test_annotations {
        assert_eq!(a.annotation_type, AnnotationType::Test);
    }
}

// @req FR-SCAN-002
#[test]
fn files_under_source_path_classified_as_impl() {
    let base = Path::new("fixtures/scan_project");
    let source = base.join("src");
    let tests = base.join("tests");
    let (annotations, _) = scan_files(&source, &tests, base);

    let impl_annotations: Vec<_> = annotations
        .iter()
        .filter(|a| a.file.starts_with("src/"))
        .collect();
    assert!(!impl_annotations.is_empty());
    for a in &impl_annotations {
        assert_eq!(a.annotation_type, AnnotationType::Impl);
    }
}

// @req FR-SCAN-002
#[test]
fn test_prefix_pattern_classified_as_test() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    // test_* pattern in source dir
    std::fs::write(
        source.join("test_helper.rs"),
        "// @req FR-TEST-001\nfn test_helper() {}\n",
    )
    .unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].annotation_type, AnnotationType::Test);
}

// @req FR-SCAN-002
#[test]
fn test_suffix_pattern_classified_as_test() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    // *_test.* pattern in source dir
    std::fs::write(
        source.join("parser_test.rs"),
        "// @req FR-TEST-001\nfn parser_test() {}\n",
    )
    .unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].annotation_type, AnnotationType::Test);
}

// @req FR-SCAN-002
#[test]
fn dot_test_pattern_classified_as_test() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    // *.test.* pattern in source dir
    std::fs::write(
        source.join("app.test.ts"),
        "// @req FR-TEST-001\nfunction test() {}\n",
    )
    .unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].annotation_type, AnnotationType::Test);
}

// @req FR-SCAN-002
#[test]
fn regular_source_file_classified_as_impl() {
    let dir = tempfile::TempDir::new().unwrap();
    let source = dir.path().join("src");
    let tests = dir.path().join("tests");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&tests).unwrap();

    std::fs::write(
        source.join("main.rs"),
        "// @req FR-TEST-001\nfn main() {}\n",
    )
    .unwrap();

    let (annotations, _) = scan_files(&source, &tests, dir.path());
    assert_eq!(annotations.len(), 1);
    assert_eq!(annotations[0].annotation_type, AnnotationType::Impl);
}
