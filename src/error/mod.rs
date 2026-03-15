use std::fmt;
use std::path::PathBuf;

// @req FR-PARSE-003
#[derive(Debug)]
pub enum ParseError {
    FileNotFound {
        path: PathBuf,
        source: std::io::Error,
    },
    MalformedYaml {
        path: PathBuf,
        line: Option<usize>,
        message: String,
    },
}

// @req FR-PARSE-003
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FileNotFound { path, source } => {
                write!(f, "File not found: {}: {}", path.display(), source)
            }
            ParseError::MalformedYaml {
                path,
                line,
                message,
            } => match line {
                Some(line) => {
                    write!(
                        f,
                        "Malformed YAML in {} at line {}: {}",
                        path.display(),
                        line,
                        message
                    )
                }
                None => {
                    write!(f, "Malformed YAML in {}: {}", path.display(), message)
                }
            },
        }
    }
}
