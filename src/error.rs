use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub enum Error {
    NotANumber,
    OutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::NotANumber => "Not a number",
            Error::OutOfBounds => "Out of bounds",
        })
    }
}

// This is important for other errors to wrap this one.
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;
