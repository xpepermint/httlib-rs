use std::fmt;
use std::error;
use httlib_huffman::{EncoderError as HuffmanError};

/// Contains error options that can be encountered while performing the encoding
/// of an HPACK header set.
#[derive(Debug, PartialEq)]
pub enum EncoderError {
    /// Indicates that the encoder received an invalid ASCII character and is
    /// thus unable to perform the (Huffman) encoding. Note that only ASCII
    /// characters provided in the HPACK spec should be used.
    InvalidInput,

    /// Indicates that an invalid index was provided. According to the HPACK
    /// specification, the index `0` must be treated as an invalid index number.
    /// The first valid number is `1`.
    InvalidIndex,

    /// Indicates that an invalid prefix was provided (must be [1, 8]).
    InvalidPrefix,

    /// Indicates that the value of the integer being encoded exceeds a certain
    /// threshold (5 bytes are chosen by this implementation). This can also
    /// happen while encoding too long string.
    IntegerOverflow,
}

impl From<HuffmanError> for EncoderError {
    fn from(err: HuffmanError) -> Self {
        match err {
            HuffmanError::InvalidInput => Self::InvalidInput
        }
    }
}

impl fmt::Display for EncoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(fmt, "Invalid input character."),
            Self::InvalidIndex => write!(fmt, "Invalid index."),
            Self::InvalidPrefix => write!(fmt, "Invalid prefix."),
            Self::IntegerOverflow => write!(fmt, "Too many bytes."),
        }
    }
}

impl error::Error for EncoderError {}
