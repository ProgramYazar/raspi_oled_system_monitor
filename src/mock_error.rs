use std::fmt;

#[derive(Debug)]
pub struct MockError {
    details: String,
}

impl MockError {
    pub fn new(msg: String) -> Self {
        Self { details: msg }
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for MockError {
    fn description(&self) -> &str {
        &self.details
    }
}
