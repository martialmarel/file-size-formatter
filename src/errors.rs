#[derive(Debug)]
pub enum Error {
    NotArgumentSizeAndUnitProvided(String),
    InvalidParsedSizeAndUnit(String),
    InvalidParsedSize(String),
    InvalidParsedUnit(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NotArgumentSizeAndUnitProvided(s) => {
                write!(f, "NotArgumentSizeAndUnitProvided: {}", s)
            }
            Error::InvalidParsedSizeAndUnit(s) => {
                write!(f, "InvalidParsedSizeAndUnit: {}", s)
            }
            Error::InvalidParsedSize(s) => {
                write!(f, "InvalidParsedSize: {}", s)
            }
            Error::InvalidParsedUnit(s) => {
                write!(f, "InvalidParsedUnit: {}", s)
            }
        }
    }
}
