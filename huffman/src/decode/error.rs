use std::error;
use std::fmt;

/// An object returned by the decode function when an error occurs.
#[derive(Debug)]
pub enum DecoderError {
    /// Indicates that the decoder received an invalid Huffman code. This should
    /// never happen in the input is encoded according to the HPACK spec.
    InvalidInput,

    /// Indicates that the available space has been filled in full. This happens
    /// when the decode function is not called often enough. Call the decode
    /// function immediately after the buffer is appended to avoid this issue.
    BufferOverflow,

    /// Indicates that the decoder received an invalid value for the speed 
    /// attribute. This attribute tells the encoder how many bits at a time
    /// should be read to decode a sequence. The expected value is between
    /// 1 and 5.
    InvalidSpeed,
}

impl fmt::Display for DecoderError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(fmt, "Invalid Huffman sequence."),
            Self::BufferOverflow => write!(fmt, "Buffer size exceeded."),
            Self::InvalidSpeed => write!(fmt, "Speed must be be between 1 and 5."),
        }
    }
}

impl error::Error for DecoderError {}
