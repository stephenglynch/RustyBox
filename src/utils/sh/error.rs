use std::fmt;
use nom::error::{ParseError, ErrorKind};


pub type RbResult<I, O, E = RbError> = Result<(I, O), nom::Err<E>>;


#[derive(Debug, PartialEq)]
pub struct RbError {
    pub code: ErrorKind
}

impl RbError {
    pub fn new(code: ErrorKind) -> Self {
        RbError {code: code}
    }
}

impl<I> ParseError<I> for RbError {
    fn from_error_kind(_: I, kind: ErrorKind) -> Self {
        Self {code: kind}
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl fmt::Display for RbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error {:?}", self.code)
    }
}