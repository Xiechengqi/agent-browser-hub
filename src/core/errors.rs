use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    ValidationError(String),
    BrowserError(String),
    TemplateError(String),
    StepError(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ExecutionError::BrowserError(msg) => write!(f, "Browser error: {}", msg),
            ExecutionError::TemplateError(msg) => write!(f, "Template error: {}", msg),
            ExecutionError::StepError(msg) => write!(f, "Step error: {}", msg),
        }
    }
}

impl std::error::Error for ExecutionError {}
