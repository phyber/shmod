// errors
use std::error;
use std::fmt;
use std::num;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidModeString,
    InvalidOctalChar(char),
    OctalDigitTooLarge(usize),
    OctalStringParseError(num::ParseIntError),
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Error {
        Error::OctalStringParseError(error)
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}", self)
    }
}
