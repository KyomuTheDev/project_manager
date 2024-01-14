use std::error::Error;

#[derive(Debug)]
pub struct ProjectError {
    details: String,
}

impl ProjectError {
    pub fn new(details: &str) -> Self {
        Self {
            details: details.to_string(),
        }
    }
}

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Project Error: {}", self.details)
    }
}

impl Error for ProjectError {
    fn description(&self) -> &str {
        &self.details
    }
}
