use super::primitives::*;

/// Provides decoder output format options.
/// 
/// This is a list of all binary formats supported by the decoder.
/// 
/// Note that bytes held by each key are considered as "safe", thus we should
/// pass only valid vectors as a key argument when instantiating this object.
/// According to this fact, the `std::convert::From` trait is imlemented instead
/// of `std::convert::TryFrom`.
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
            DecoderLit::BoolList(byt) => decode_bool_list(&byt, &mut dst).unwrap_or(0),
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
            DecoderLit::Int32List(byt) => decode_int32_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SInt32List(byt) => decode_sint32_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SFixed32List(byt) => decode_sfixed32_list(&byt, &mut dst).unwrap_or(0),
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
            DecoderLit::Int64List(byt) => decode_int64_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SInt64List(byt) => decode_sint64_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::SFixed64List(byt) => decode_sfixed64_list(&byt, &mut dst).unwrap_or(0),
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
            DecoderLit::UInt32List(byt) => decode_uint32_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::Fixed32List(byt) => decode_fixed32_list(&byt, &mut dst).unwrap_or(0),
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
            DecoderLit::UInt64List(byt) => decode_uint64_list(&byt, &mut dst).unwrap_or(0),
            DecoderLit::Fixed64List(byt) => decode_fixed64_list(&byt, &mut dst).unwrap_or(0),
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
            DecoderLit::FloatList(byt) => decode_float_list(&byt, &mut dst).unwrap_or(0),
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
            DecoderLit::DoubleList(byt) => decode_double_list(&byt, &mut dst).unwrap_or(0),
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
