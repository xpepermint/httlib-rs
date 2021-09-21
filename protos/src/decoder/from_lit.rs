use super::{DecoderError, DecoderLit};
use super::primitives::*;

/// Provides an implementation interface for Rust data types to support
/// conversion to `DecoderLit` instance.
pub trait FromDecoderLit {
    /// Creates a new instance from the `DecoderLit` instance.
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError>
        where
        Self: std::marker::Sized;
}

impl FromDecoderLit for bool {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = false;
        match lit {
            DecoderLit::Bool(byt) => decode_bool(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<bool> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::BoolList(byt) => decode_bool_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for i32 {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = 0i32;
        match lit {
            DecoderLit::Int32(byt) => decode_int32(&byt, &mut dst)?,
            DecoderLit::SInt32(byt) => decode_sint32(&byt, &mut dst)?,
            DecoderLit::SFixed32(byt) => decode_sfixed32(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<i32> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::Int32List(byt) => decode_int32_list(&byt, &mut dst)?,
            DecoderLit::SInt32List(byt) => decode_sint32_list(&byt, &mut dst)?,
            DecoderLit::SFixed32List(byt) => decode_sfixed32_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for i64 {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = 0i64;
        match lit {
            DecoderLit::Int64(byt) => decode_int64(&byt, &mut dst)?,
            DecoderLit::SInt64(byt) => decode_sint64(&byt, &mut dst)?,
            DecoderLit::SFixed64(byt) => decode_sfixed64(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<i64> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::Int64List(byt) => decode_int64_list(&byt, &mut dst)?,
            DecoderLit::SInt64List(byt) => decode_sint64_list(&byt, &mut dst)?,
            DecoderLit::SFixed64List(byt) => decode_sfixed64_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for u32 {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = 0u32;
        match lit {
            DecoderLit::UInt32(byt) => decode_uint32(&byt, &mut dst)?,
            DecoderLit::Fixed32(byt) => decode_fixed32(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<u32> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::UInt32List(byt) => decode_uint32_list(&byt, &mut dst)?,
            DecoderLit::Fixed32List(byt) => decode_fixed32_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for u64 {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = 0u64;
        match lit {
            DecoderLit::UInt64(byt) => decode_uint64(&byt, &mut dst)?,
            DecoderLit::Fixed64(byt) => decode_fixed64(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<u64> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::UInt64List(byt) => decode_uint64_list(&byt, &mut dst)?,
            DecoderLit::Fixed64List(byt) => decode_fixed64_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for f32 {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = 0.0f32;
        match lit {
            DecoderLit::Float(byt) => decode_float(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<f32> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::FloatList(byt) => decode_float_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for f64 {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = 0.0f64;
        match lit {
            DecoderLit::Double(byt) => decode_double(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<f64> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        let mut dst = vec![];
        match lit {
            DecoderLit::DoubleList(byt) => decode_double_list(&byt, &mut dst)?,
            _ => return Err(DecoderError::InvalidInput),
        };
        Ok(dst)
    }
}

impl FromDecoderLit for Vec<u8> {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        match lit {
            DecoderLit::Bytes(byt) => Ok(byt),
            _ => Err(DecoderError::InvalidInput),
        }
    }
}

impl FromDecoderLit for String {
    fn from_decoder_lit(lit: DecoderLit) -> Result<Self, DecoderError> {
        match lit {
            DecoderLit::Bytes(byt) => {
                match String::from_utf8(byt) {
                    Ok(s) => Ok(s),
                    _ => Err(DecoderError::InvalidInput),
                }
            },
            _ => Err(DecoderError::InvalidInput),
        }
    }
}
