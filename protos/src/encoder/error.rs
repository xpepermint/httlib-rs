use std::error;
use std::fmt;
use std::io;

/// Contains error options that can be encountered while performing the encoding
/// operations.
#[derive(Debug, PartialEq)]
pub enum EncoderError {
    /// Indicates that the data size limit has been reached.
    DataOverflow,

    /// Indicates that the encoder encountered an I/O interruption. Interrupted
    /// operations can typically be retried.
    Interrupted,
    
    /// Indicates that the encoder was unable to proceed due to the key's
    /// invalid tag number. A tag number must be unique per message and the
    /// value can be between `0` and `2^29 - 1`.
    InvalidTag,
}

impl From<io::Error> for EncoderError {
    fn from(_err: io::Error) -> Self {
        Self::Interrupted
    }
}

impl fmt::Display for EncoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DataOverflow => write!(fmt, "Available data type size exceeded."),
            Self::Interrupted => write!(fmt, "Write operation interrupted."),
            Self::InvalidTag => write!(fmt, "Found tag with invalid number."),
        }
    }
}

impl error::Error for EncoderError {}
