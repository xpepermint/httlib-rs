use std::error;
use std::fmt;

/// An object returned by the decode function when an error occurs.
#[derive(Debug, PartialEq)]
pub enum DecoderError {
    /// Indicates that the decoder received an invalid Huffman code. This should
    /// never happen in the input is encoded according to the HPACK spec.
    InvalidInput,
}

impl fmt::Display for DecoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(fmt, "Invalid Huffman sequence."),
        }
    }
}

impl error::Error for DecoderError {}
