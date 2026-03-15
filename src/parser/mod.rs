use std::path::Path;

use crate::models::Requirement;

// @req FR-PARSE-001
#[derive(serde::Deserialize)]
struct RequirementsFile {
    requirements: Vec<Requirement>,
}

// @req FR-PARSE-001
pub fn parse_requirements(path: &Path) -> Result<Vec<Requirement>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let file: RequirementsFile = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))?;

    Ok(file.requirements)
}
