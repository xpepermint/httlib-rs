use httlib_huffman;

/// Represents all errors that can be encountered while performing the encoding
/// of an HPACK header set.
#[derive(Debug, PartialEq)]
pub enum EncoderError {
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
    /// Represents all errors that can be encoiuntered by the Huffman encoder.
    Huffman(httlib_huffman::EncodeError),
}
