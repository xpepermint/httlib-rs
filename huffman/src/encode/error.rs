use std::error;
use std::fmt;

/// An object returned by the encode function when an error occurs.
#[derive(Debug)]
pub enum EncodeError {
    /// Indicates that the encoder received an invalid ASCII character. Note
    /// that only ASCII characters provided in the HPACK spec should be used.
    InvalidCharacter,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidCharacter => write!(fmt, "Failed to encode a character."),
        }
    }
}

impl error::Error for EncodeError {}
