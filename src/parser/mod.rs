use std::path::Path;

use crate::models::{Requirement, Task};

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

// @req FR-PARSE-002
#[derive(serde::Deserialize)]
struct TasksFile {
    tasks: Vec<Task>,
}

// @req FR-PARSE-002
pub fn parse_tasks(requirements_path: &Path) -> Result<Vec<Task>, String> {
    let tasks_path = requirements_path
        .parent()
        .map(|p| p.join("tasks.yaml"))
        .unwrap_or_else(|| "tasks.yaml".into());

    let content = std::fs::read_to_string(&tasks_path)
        .map_err(|e| format!("Failed to read {}: {}", tasks_path.display(), e))?;

    let file: TasksFile = serde_yaml::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", tasks_path.display(), e))?;

    Ok(file.tasks)
}
