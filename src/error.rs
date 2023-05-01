use std::fmt;
use std::error;

#[derive(Debug)]
pub struct RustyBoxError {
    details: String
}

impl RustyBoxError {
    pub fn new(msg: &str) -> RustyBoxError {
        RustyBoxError { details: msg.to_string() }
    }
}

impl fmt::Display for RustyBoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for RustyBoxError {
    fn description(&self) -> &str {
        &self.details
    }
}