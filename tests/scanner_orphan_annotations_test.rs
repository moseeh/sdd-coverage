use std::collections::HashSet;

use sdd_coverage::models::{Annotation, AnnotationType};
use sdd_coverage::scanner::find_orphan_annotations;

fn make_annotation(req_id: &str) -> Annotation {
    Annotation {
        file: "src/main.rs".to_string(),
        line: 1,
        req_id: req_id.to_string(),
        annotation_type: AnnotationType::Impl,
        snippet: "// @req ".to_string() + req_id,
    }
}

// @req FR-SCAN-004
#[test]
fn detects_orphan_annotations() {
    let annotations = vec![
        make_annotation("FR-EXISTS-001"),
        make_annotation("FR-ORPHAN-001"),
        make_annotation("FR-EXISTS-002"),
    ];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001", "FR-EXISTS-002"].into();

    let orphans = find_orphan_annotations(&annotations, &req_ids);
    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0].req_id, "FR-ORPHAN-001");
}

// @req FR-SCAN-004
#[test]
fn no_orphans_when_all_match() {
    let annotations = vec![
        make_annotation("FR-EXISTS-001"),
        make_annotation("FR-EXISTS-002"),
    ];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001", "FR-EXISTS-002"].into();

    let orphans = find_orphan_annotations(&annotations, &req_ids);
    assert!(orphans.is_empty());
}

// @req FR-SCAN-004
#[test]
fn all_orphans_when_none_match() {
    let annotations = vec![
        make_annotation("FR-ORPHAN-001"),
        make_annotation("FR-ORPHAN-002"),
    ];
    let req_ids: HashSet<&str> = ["FR-OTHER-001"].into();

    let orphans = find_orphan_annotations(&annotations, &req_ids);
    assert_eq!(orphans.len(), 2);
}

// @req FR-SCAN-004
#[test]
fn empty_annotations_returns_empty() {
    let annotations: Vec<Annotation> = vec![];
    let req_ids: HashSet<&str> = ["FR-EXISTS-001"].into();

    let orphans = find_orphan_annotations(&annotations, &req_ids);
    assert!(orphans.is_empty());
}

// @req FR-SCAN-004
#[test]
fn empty_requirements_makes_all_orphans() {
    let annotations = vec![make_annotation("FR-ANY-001")];
    let req_ids: HashSet<&str> = HashSet::new();

    let orphans = find_orphan_annotations(&annotations, &req_ids);
    assert_eq!(orphans.len(), 1);
}
