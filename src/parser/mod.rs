use std::collections::HashSet;
use std::path::Path;

use crate::error::ParseError;
use crate::models::{Requirement, Task};

// @req FR-PARSE-001
#[derive(serde::Deserialize)]
struct RequirementsFile {
    requirements: Vec<Requirement>,
}

// @req FR-PARSE-003
fn read_file(path: &Path) -> Result<String, ParseError> {
    std::fs::read_to_string(path).map_err(|e| ParseError::FileNotFound {
        path: path.to_path_buf(),
        source: e,
    })
}

// @req FR-PARSE-003
fn deserialize_yaml<T: serde::de::DeserializeOwned>(
    content: &str,
    path: &Path,
) -> Result<T, ParseError> {
    serde_yaml::from_str(content).map_err(|e| ParseError::MalformedYaml {
        path: path.to_path_buf(),
        line: e.location().map(|loc| loc.line()),
        message: e.to_string(),
    })
}

// @req FR-PARSE-001
pub fn parse_requirements(path: &Path) -> Result<Vec<Requirement>, ParseError> {
    let content = read_file(path)?;
    let file: RequirementsFile = deserialize_yaml(&content, path)?;
    check_duplicate_ids(file.requirements.iter().map(|r| &r.id), path)?;
    Ok(file.requirements)
}

// @req FR-PARSE-002
#[derive(serde::Deserialize)]
struct TasksFile {
    tasks: Vec<Task>,
}

// @req FR-PARSE-002
pub fn parse_tasks(requirements_path: &Path) -> Result<Vec<Task>, ParseError> {
    let tasks_path = requirements_path
        .parent()
        .map(|p| p.join("tasks.yaml"))
        .unwrap_or_else(|| "tasks.yaml".into());

    let content = read_file(&tasks_path)?;
    let file: TasksFile = deserialize_yaml(&content, &tasks_path)?;
    check_duplicate_ids(file.tasks.iter().map(|t| &t.id), &tasks_path)?;
    Ok(file.tasks)
}

// @req FR-PARSE-004
fn check_duplicate_ids<'a>(
    ids: impl Iterator<Item = &'a String>,
    path: &Path,
) -> Result<(), ParseError> {
    let mut seen = HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(ParseError::DuplicateId {
                id: id.clone(),
                path: path.to_path_buf(),
            });
        }
    }
    Ok(())
}
