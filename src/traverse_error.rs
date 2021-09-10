use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TraverseError {
    message: String
}

impl TraverseError {
    pub fn new<S: AsRef<str>>(details: S) -> Self {
        Self {
            message: details.as_ref().to_string()
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for TraverseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for TraverseError {
    fn description(&self) -> &str {
        &self.message
    }
}