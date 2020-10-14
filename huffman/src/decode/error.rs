use std::error;
use std::fmt;

/// An object returned by the decode function when an error occurs.
#[derive(Debug)]
pub enum DecoderError {
    /// Indicates that the decoder received an invalid Huffman code. This should
    /// never happen in the input is encoded according to the HPACK spec.
    InvalidHuffmanCode,

    /// Indicates that the available space has been filled in full. This happens
    /// when the decode function is not called often enough. Call the decode
    /// function immediately after the buffer is appended to avoid this issue.
    BufferOverflow,
}

impl fmt::Display for DecoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidHuffmanCode => write!(fmt, "Failed to decode an invalid Huffman code."),
            Self::BufferOverflow => write!(fmt, "Buffer size exceeded."),
        }
    }
}

impl error::Error for DecoderError {}
