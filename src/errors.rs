pub enum Error {
    NotArgumentSizeAndUnitProvided(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NotArgumentSizeAndUnitProvided(s) => {
                write!(f, "NotArgumentSizeAndUnitProvided: {}", s)
            }
        }
    }
}
