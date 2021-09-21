use super::{DecoderError, FromDecoderLit};

/// Provides decoder output format options.
/// 
/// This is a list of all binary formats supported by the decoder.
#[derive(Debug)]
pub enum DecoderLit {
    /// Represents `binary` format of wire type `2`.
    Bytes(Vec<u8>),

    /// Represents `bool` format of wire type `0`.
    Bool(Vec<u8>),

    /// Represents `bool` format of wire type `2` for packed repeated fields.
    BoolList(Vec<u8>),

    /// Represents `int32` format of wire type `0`.
    Int32(Vec<u8>),

    /// Represents `int32` format of wire type `0` for packed repeated fields.
    Int32List(Vec<u8>),

    /// Represents `int64` format of wire type `0`.
    Int64(Vec<u8>),

    /// Represents `int64` format of wire type `0` for packed repeated fields.
    Int64List(Vec<u8>),

    /// Represents `uint32` format of wire type `0`.
    UInt32(Vec<u8>),

    /// Represents `uint32` format of wire type `0` for packed repeated fields.
    UInt32List(Vec<u8>),

    /// Represents `uint64` format of wire type `0`.
    UInt64(Vec<u8>),

    /// Represents `uint64` format of wire type `0` for packed repeated fields.
    UInt64List(Vec<u8>),

    /// Represents `float` format of wire type `5`.
    Float(Vec<u8>),

    /// Represents `float` format of wire type `5` for packed repeated fields.
    FloatList(Vec<u8>),

    /// Represents `uint32` format of wire type `1`.
    Double(Vec<u8>),

    /// Represents `double` format of wire type `1` for packed repeated fields.
    DoubleList(Vec<u8>),

    /// Represents `sint32` format of wire type `0`.
    SInt32(Vec<u8>),

    /// Represents `sint32` format of wire type `0` for packed repeated fields.
    SInt32List(Vec<u8>),

    /// Represents `sint64` format of wire type `0`.
    SInt64(Vec<u8>),

    /// Represents `sint64` format of wire type `0` for packed repeated fields.
    SInt64List(Vec<u8>),

    /// Represents `fixed32` format of wire type `5`.
    Fixed32(Vec<u8>),

    /// Represents `fixed32` format of wire type `5` for packed repeated fields.
    Fixed32List(Vec<u8>),

    /// Represents `fixed64` format of wire type `1`.
    Fixed64(Vec<u8>),

    /// Represents `fixed64` format of wire type `1` for packed repeated fields.
    Fixed64List(Vec<u8>),

    /// Represents `sfixed32` format of wire type `5`.
    SFixed32(Vec<u8>),

    /// Represents `sfixed32` format of wire type `5` for packed repeated fields.
    SFixed32List(Vec<u8>),

    /// Represents `sfixed64` format of wire type `1`.
    SFixed64(Vec<u8>),

    /// Represents `sfixed64` format of wire type `1` for packed repeated
    /// fields.
    SFixed64List(Vec<u8>),
}

impl DecoderLit {
    pub fn parse<T: FromDecoderLit>(self) -> Result<T, DecoderError> {
        FromDecoderLit::from_decoder_lit(self)
    }
}
