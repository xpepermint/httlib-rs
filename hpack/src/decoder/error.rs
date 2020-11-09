use std::fmt;
use std::error;
use httlib_huffman::{DecoderError as HuffmanError};

/// Represents all errors that can be encountered while performing the decoding
/// of an HPACK header set.
#[derive(Debug, PartialEq)]
pub enum DecoderError {
    /// Indicates that the decoder received an invalid (Huffman) buffer. This
    /// should never happen if the input is encoded according to the HPACK spec.
    InvalidInput,
    /// Indicates that an invalid index was provided. According to the HPACK
    /// specification, the index `0` must be treated as an invalid index number.
    /// The first valid number is `1`.
    InvalidIndex,
    /// Indicates that an invalid prefix was provided (must be [1, 8]).
    InvalidPrefix,
    /// Indicates that the value of the integer being decoded exceeds a certain
    /// threshold (5 bytes are chosen by this implementation).
    IntegerOverflow,
    /// Indicates that the buffer from which an integer was supposed to be
    /// decode does not contain enough octets to complete the decoding.
    IntegerUnderflow,
    /// Indicates that the decoder received a size that do not follow external
    /// protocol rules.
    InvalidMaxDynamicSize,
}

impl From<HuffmanError> for DecoderError {
    fn from(err: HuffmanError) -> Self {
        match err {
            HuffmanError::InvalidInput => Self::InvalidInput
        }
    }
}

impl fmt::Display for DecoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(fmt, "Invalid input character."),
            Self::InvalidIndex => write!(fmt, "Invalid index."),
            Self::InvalidPrefix => write!(fmt, "Invalid prefix."),
            Self::IntegerOverflow => write!(fmt, "Too many bytes."),
            Self::IntegerUnderflow => write!(fmt, "Not enough bytes."),
            Self::InvalidMaxDynamicSize => write!(fmt, "New size exceeds hard limit."),
        }
    }
}

impl error::Error for DecoderError {}
