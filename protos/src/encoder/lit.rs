/// Provides encoder input format options.
/// 
/// This is a list of all binary formats supported by the encoder.
#[derive(Debug)]
pub enum EncoderLit {
    /// Represents `binary` format of wire type `2`.
    Bytes(Vec<u8>),

    /// Represents `bool` format of wire type `0`.
    Bool(bool),

    /// Represents `bool` format of wire type `2` for packed repeated fields.
    BoolList(Vec<bool>),

    /// Represents `int32` format of wire type `0`.
    Int32(i32),

    /// Represents `int32` format of wire type `0` for packed repeated fields.
    Int32List(Vec<i32>),

    /// Represents `int64` format of wire type `0`.
    Int64(i64),

    /// Represents `int64` format of wire type `0` for packed repeated fields.
    Int64List(Vec<i64>),

    /// Represents `uint32` format of wire type `0`.
    UInt32(u32),

    /// Represents `uint32` format of wire type `0` for packed repeated fields.
    UInt32List(Vec<u32>),

    /// Represents `uint64` format of wire type `0`.
    UInt64(u64),

    /// Represents `uint64` format of wire type `0` for packed repeated fields.
    UInt64List(Vec<u64>),

    /// Represents `float` format of wire type `5`.
    Float(f32),

    /// Represents `float` format of wire type `5` for packed repeated fields.
    FloatList(Vec<f32>),

    /// Represents `uint32` format of wire type `1`.
    Double(f64),

    /// Represents `double` format of wire type `1` for packed repeated fields.
    DoubleList(Vec<f64>),

    /// Represents `sint32` format of wire type `0`.
    SInt32(i32),

    /// Represents `sint32` format of wire type `0` for packed repeated fields.
    SInt32List(Vec<i32>),

    /// Represents `sint64` format of wire type `0`.
    SInt64(i64),

    /// Represents `sint64` format of wire type `0` for packed repeated fields.
    SInt64List(Vec<i64>),

    /// Represents `fixed32` format of wire type `5`.
    Fixed32(u32),

    /// Represents `fixed32` format of wire type `5` for packed repeated fields.
    Fixed32List(Vec<u32>),

    /// Represents `fixed64` format of wire type `1`.
    Fixed64(u64),

    /// Represents `fixed64` format of wire type `1` for packed repeated fields.
    Fixed64List(Vec<u64>),

    /// Represents `sfixed32` format of wire type `5`.
    SFixed32(i32),

    /// Represents `sfixed32` format of wire type `5` for packed repeated
    /// fields.
    SFixed32List(Vec<i32>),

    /// Represents `sfixed64` format of wire type `1`.
    SFixed64(i64),

    /// Represents `sfixed64` format of wire type `1` for packed repeated
    /// fields.
    SFixed64List(Vec<i64>),
}

impl From<bool> for EncoderLit {
    fn from(val: bool) -> Self {
        Self::Bool(val)
    }
}

impl From<Vec<bool>> for EncoderLit {
    fn from(val: Vec<bool>) -> Self {
        Self::BoolList(val)
    }
}

impl From<i32> for EncoderLit {
    fn from(val: i32) -> Self {
        Self::Int32(val)
    }
}

impl From<Vec<i32>> for EncoderLit {
    fn from(val: Vec<i32>) -> Self {
        Self::Int32List(val)
    }
}

impl From<i64> for EncoderLit {
    fn from(val: i64) -> Self {
        Self::Int64(val)
    }
}

impl From<Vec<i64>> for EncoderLit {
    fn from(val: Vec<i64>) -> Self {
        Self::Int64List(val)
    }
}

impl From<u32> for EncoderLit {
    fn from(val: u32) -> Self {
        Self::UInt32(val)
    }
}

impl From<Vec<u32>> for EncoderLit {
    fn from(val: Vec<u32>) -> Self {
        Self::UInt32List(val)
    }
}

impl From<u64> for EncoderLit {
    fn from(val: u64) -> Self {
        Self::UInt64(val)
    }
}

impl From<Vec<u64>> for EncoderLit {
    fn from(val: Vec<u64>) -> Self {
        Self::UInt64List(val)
    }
}

impl From<f32> for EncoderLit {
    fn from(val: f32) -> Self {
        Self::Float(val)
    }
}

impl From<Vec<f32>> for EncoderLit {
    fn from(val: Vec<f32>) -> Self {
        Self::FloatList(val)
    }
}

impl From<f64> for EncoderLit {
    fn from(val: f64) -> Self {
        Self::Double(val)
    }
}

impl From<Vec<f64>> for EncoderLit {
    fn from(val: Vec<f64>) -> Self {
        Self::DoubleList(val)
    }
}

impl From<Vec<u8>> for EncoderLit {
    fn from(val: Vec<u8>) -> Self {
        Self::Bytes(val)
    }
}
