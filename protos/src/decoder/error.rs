use std::error;
use std::fmt;
use std::io;

/// Contains error options that can be encountered while performing the decoding
/// operations.
#[derive(Debug, PartialEq)]
pub enum DecoderError {
    /// Indicates that the decoder received an invalid stream of bytes that can
    /// not be decoded.
    InvalidInput,

    /// Indicates that the decoder encountered an I/O interruption. Interrupted
    /// operations can typically be retried.
    Interrupted,

    /// Indicates that the buffer from which an item was supposed to be decode
    /// does not contain enough octets to complete the decoding.
    InputUnderflow,

    /// Indicates that the decoder encountered an invalid tag number of a key. 
    /// A tag number must be unique per message and the value can be between `0`
    /// and `2^29 - 1`.
    InvalidTag,
}

impl From<io::Error> for DecoderError {
    fn from(_err: io::Error) -> Self {
        Self::Interrupted
    }
}

impl fmt::Display for DecoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InputUnderflow => write!(fmt, "Not enough bytes."),
            Self::Interrupted => write!(fmt, "Read operation interrupted."),
            Self::InvalidInput => write!(fmt, "Invalid byte stream."),
            Self::InvalidTag => write!(fmt, "Found tag with invalid number."),
        }
    }
}

impl error::Error for DecoderError {}
