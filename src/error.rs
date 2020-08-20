// errors
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum Error {
    OctalDigitTooLarge(usize),
    OctalStringParseError(num::ParseIntError),
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Error {
        Error::OctalStringParseError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {:?}", self)
    }
}
