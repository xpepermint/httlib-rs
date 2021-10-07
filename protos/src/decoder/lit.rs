use super::primitives::*;

/// Provides decoder output format options.
/// 
/// This is a list of all binary formats supported by the decoder.
/// 
/// Note that bytes held by each key are considered "safe". Therefore, we should
/// only pass valid vectors as key arguments when instantiating this object.
/// According to this fact, the 'std::convert::From' property is implemented
/// instead of 'std::convert::TryFrom'.
#[derive(Debug)]
pub enum DecoderLit {
    /// Represents `binary` format of wire type `2`.
    Bytes(Vec<u8>),

    /// Represents `bool` format of wire type `0`.
    Bool(Vec<u8>),

    /// Represents `bool` format of wire type `2` for packed repeated fields.
    BoolVec(Vec<u8>),

    /// Represents `int32` format of wire type `0`.
    Int32(Vec<u8>),

    /// Represents `int32` format of wire type `0` for packed repeated fields.
    Int32Vec(Vec<u8>),

    /// Represents `int64` format of wire type `0`.
    Int64(Vec<u8>),

    /// Represents `int64` format of wire type `0` for packed repeated fields.
    Int64Vec(Vec<u8>),

    /// Represents `uint32` format of wire type `0`.
    UInt32(Vec<u8>),

    /// Represents `uint32` format of wire type `0` for packed repeated fields.
    UInt32Vec(Vec<u8>),

    /// Represents `uint64` format of wire type `0`.
    UInt64(Vec<u8>),

    /// Represents `uint64` format of wire type `0` for packed repeated fields.
    UInt64Vec(Vec<u8>),

    /// Represents `float` format of wire type `5`.
    Float(Vec<u8>),

    /// Represents `float` format of wire type `5` for packed repeated fields.
    FloatVec(Vec<u8>),

    /// Represents `uint32` format of wire type `1`.
    Double(Vec<u8>),

    /// Represents `double` format of wire type `1` for packed repeated fields.
    DoubleVec(Vec<u8>),

    /// Represents `sint32` format of wire type `0`. Use it when the value is
    /// likely to be negative.
    SInt32(Vec<u8>),

    /// Represents `sint32` format of wire type `0` for packed repeated fields.
    /// Use it when the values are likely to be negative.
    SInt32Vec(Vec<u8>),

    /// Represents `sint64` format of wire type `0`. Use it when the value is
    /// likely to be negative.
    SInt64(Vec<u8>),

    /// Represents `sint64` format of wire type `0` for packed repeated fields.
    /// Use it when the values are likely to be negative.
    SInt64Vec(Vec<u8>),

    /// Represents `fixed32` format of wire type `5`.
    Fixed32(Vec<u8>),

    /// Represents `fixed32` format of wire type `5` for packed repeated fields.
    Fixed32Vec(Vec<u8>),

    /// Represents `fixed64` format of wire type `1`.
    Fixed64(Vec<u8>),

    /// Represents `fixed64` format of wire type `1` for packed repeated fields.
    Fixed64Vec(Vec<u8>),

    /// Represents `sfixed32` format of wire type `5`.
    SFixed32(Vec<u8>),

    /// Represents `sfixed32` format of wire type `5` for packed repeated fields.
    SFixed32Vec(Vec<u8>),

    /// Represents `sfixed64` format of wire type `1`.
    SFixed64(Vec<u8>),

    /// Represents `sfixed64` format of wire type `1` for packed repeated
    /// fields.
    SFixed64Vec(Vec<u8>),
}

impl From<DecoderLit> for bool {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = false;
        match lit {
            DecoderLit::Bool(byt) => decode_bool(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<bool> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::BoolVec(byt) => decode_bool_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for i32 {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = 0i32;
        match lit {
            DecoderLit::Int32(byt) => decode_int32(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SInt32(byt) => decode_sint32(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SFixed32(byt) => decode_sfixed32(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<i32> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::Int32Vec(byt) => decode_int32_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SInt32Vec(byt) => decode_sint32_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SFixed32Vec(byt) => decode_sfixed32_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for i64 {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = 0i64;
        match lit {
            DecoderLit::Int64(byt) => decode_int64(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SInt64(byt) => decode_sint64(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SFixed64(byt) => decode_sfixed64(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<i64> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::Int64Vec(byt) => decode_int64_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SInt64Vec(byt) => decode_sint64_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SFixed64Vec(byt) => decode_sfixed64_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for u32 {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = 0u32;
        match lit {
            DecoderLit::UInt32(byt) => decode_uint32(&byt, &mut dst).unwrap_or(0),
            DecoderLit::Fixed32(byt) => decode_fixed32(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<u32> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::UInt32Vec(byt) => decode_uint32_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::Fixed32Vec(byt) => decode_fixed32_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for u64 {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = 0u64;
        match lit {
            DecoderLit::UInt64(byt) => decode_uint64(&byt, &mut dst).unwrap_or(0),
            DecoderLit::Fixed64(byt) => decode_fixed64(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<u64> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::UInt64Vec(byt) => decode_uint64_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::Fixed64Vec(byt) => decode_fixed64_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for f32 {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = 0.0f32;
        match lit {
            DecoderLit::Float(byt) => decode_float(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<f32> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::FloatVec(byt) => decode_float_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for f64 {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = 0.0f64;
        match lit {
            DecoderLit::Double(byt) => decode_double(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<f64> {
    fn from(lit: DecoderLit) -> Self {
        let mut dst = vec![];
        match lit {
            DecoderLit::DoubleVec(byt) => decode_double_list(&byt, &mut dst).unwrap_or(0),
            _ => return dst,
        };
        dst
    }
}

impl From<DecoderLit> for Vec<u8> {
    fn from(lit: DecoderLit) -> Self {
        match lit {
            DecoderLit::Bytes(byt) => byt,
            _ => Vec::new(),
        }
    }
}

impl From<DecoderLit> for String {
    fn from(lit: DecoderLit) -> Self {
        match lit {
            DecoderLit::Bytes(byt) => {
                match String::from_utf8(byt) {
                    Ok(s) => s,
                    _ => String::new(),
                }
            },
            _ => String::new(),
        }
    }
}
