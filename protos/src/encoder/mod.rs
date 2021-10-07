//! Provides an implementation of the `proto3` encoder.
//! 
//! The encoder performs the task of transforming data fields into shrunken
//! binary format. This sequence of bytes is much smaller than the original
//! message which allows for much faster data transmission over the wire.
//! 
//! Property names are represented in the [Protocol Buffers] by unique numbers
//! rather than strings. Compared to the raw JSON format, this already has a
//! significant impact on the final size of the message that is then sent over
//! the wire.
//! 
//! ```txt
//! +-------------------+------------------+-------------------+
//! +      1. JSON      +   2. Transform   +     3. Encode     + ENCODER
//! +-------------------+------------------+-------------------+
//! + {                 +                  +                   +
//! +   "name": "John", + 1, John          + 0a 04 4a 6f 68 6e +
//! +   "age": 35       + 2, 35            + 10 23             +
//! + }                 +                  +                   +
//! +-------------------+------------------+-------------------+
//! +      6. JSON      +    5. Rebuild    +     4. Decode     + DECODER
//! +-------------------+------------------+-------------------+
//! ```
//! 
//! The encoder encodes a message into a binary format. The message is then
//! represented on the wire as a kind of flattened sequence of encoded key-value
//! properties. The key and the value are encoded separately. Each wire type has
//! its own rules and therefore its own way of encoding.
//! 
//! ```txt
//! [key1][value1][key2][value2] ... [keyN][valueN]
//! ```

mod error;
mod lit;
mod primitives;

use std::io;
use crate::Typ;
pub use error::*;
pub use lit::*;
use primitives::*;

/// Provides the encoding engine for Protocol Buffers.
pub struct Encoder;

impl Encoder {
    /// A constant holding the minimum `tag` number of a field.
    pub const TAG_MIN: u32 = 1;

    /// A constant holding the maximum `tag` number of a field.
    pub const TAG_MAX: u32 = (1 << 29) - 1;

    /// Transforms a `field` into `proto3` binary format and writes the result
    /// into `dst`.
    /// 
    /// By default, the encoder uses mostly "standard" variant formats for
    /// numbers. A developer can choose a specific format by passing the 
    /// `EncoderLit`.
    /// 
    /// ```rust
    /// use httlib_protos::{Encoder, EncoderLit};
    /// 
    /// let encoder = Encoder::default();
    /// 
    /// let mut dst = Vec::new();
    /// encoder.encode((1, &150i32), &mut dst).unwrap();
    /// encoder.encode((2, EncoderLit::SInt32(&-150i32)), &mut dst).unwrap();
    /// ```
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode<'a, W, F>(
        &self,
        field: (u32, F),
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        F: Into<EncoderLit<'a>>,
        W: ?Sized + io::Write,
    {
        match field.1.into() {
            EncoderLit::Bool(val) => self.encode_bool(field.0, val, dst),
            EncoderLit::BoolVec(val) => self.encode_bool_list(field.0, val, dst),
            EncoderLit::Int32(val) => self.encode_int32(field.0, val, dst),
            EncoderLit::Int32Vec(val) => self.encode_int32_list(field.0, val, dst),
            EncoderLit::Int64(val) => self.encode_int64(field.0, val, dst),
            EncoderLit::Int64Vec(val) => self.encode_int64_list(field.0, val, dst),
            EncoderLit::UInt32(val) => self.encode_uint32(field.0, val, dst),
            EncoderLit::UInt32Vec(val) => self.encode_uint32_list(field.0, val, dst),
            EncoderLit::UInt64(val) => self.encode_uint64(field.0, val, dst),
            EncoderLit::UInt64Vec(val) => self.encode_uint64_list(field.0, val, dst),
            EncoderLit::Float(val) => self.encode_float(field.0, val, dst),
            EncoderLit::FloatVec(val) => self.encode_float_list(field.0, val, dst),
            EncoderLit::Double(val) => self.encode_double(field.0, val, dst),
            EncoderLit::DoubleVec(val) => self.encode_double_list(field.0, val, dst),
            EncoderLit::SInt32(val) => self.encode_sint32(field.0, val, dst),
            EncoderLit::SInt32Vec(val) => self.encode_sint32_list(field.0, val, dst),
            EncoderLit::SInt64(val) => self.encode_sint64(field.0, val, dst),
            EncoderLit::SInt64Vec(val) => self.encode_sint64_list(field.0, val, dst),
            EncoderLit::Fixed32(val) => self.encode_fixed32(field.0, val, dst),
            EncoderLit::Fixed32Vec(val) => self.encode_fixed32_list(field.0, val, dst),
            EncoderLit::Fixed64(val) => self.encode_fixed64(field.0, val, dst),
            EncoderLit::Fixed64Vec(val) => self.encode_fixed64_list(field.0, val, dst),
            EncoderLit::SFixed32(val) => self.encode_sfixed32(field.0, val, dst),
            EncoderLit::SFixed32Vec(val) => self.encode_sfixed32_list(field.0, val, dst),
            EncoderLit::SFixed64(val) => self.encode_sfixed64(field.0, val, dst),
            EncoderLit::SFixed64Vec(val) => self.encode_sfixed64_list(field.0, val, dst),
            EncoderLit::Bytes(val) => self.encode_bytes(field.0, val, dst),
        }
    }

    /// Encodes the provided `val` into `bool` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_bool<W>(
        &self,
        tag: u32,
        val: &bool,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_bool(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `bool` repeated field with a specific
    /// `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_bool_list<W>(
        &self,
        tag: u32,
        vals: &Vec<bool>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_bool(*val, &mut data)?;
        }
        let mut size = 0;
        size += encode_key(tag, Typ::LengthDelimited, dst)?;
        size += encode_bytes(data, dst)?;
        Ok(size)
    }

    /// Encodes the provided `val` into `int32` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_int32<W>(
        &self,
        tag: u32,
        val: &i32,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_int32(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `int32` repeated field with a specific
    /// `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_int32_list<W>(
        &self,
        tag: u32,
        vals: &Vec<i32>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_int32(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `int64` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_int64<W>(
        &self,
        tag: u32,
        val: &i64,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_int64(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `int64` repeated field with a specific
    /// `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_int64_list<W>(
        &self,
        tag: u32,
        vals: &Vec<i64>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_int64(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `uint32` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_uint32<W>(
        &self,
        tag: u32,
        val: &u32,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_uint32(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `uint32` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_uint32_list<W>(
        &self,
        tag: u32,
        vals: &Vec<u32>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_uint32(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `uint64` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_uint64<W>(
        &self,
        tag: u32,
        val: &u64,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_uint64(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `uint64` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_uint64_list<W>(
        &self,
        tag: u32,
        vals: &Vec<u64>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_uint64(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `float` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_float<W>(
        &self,
        tag: u32,
        val: &f32,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Bit32, dst)?;
        size += encode_float(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `float` repeated field with a specific
    /// `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_float_list<W>(
        &self,
        tag: u32,
        vals: &Vec<f32>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_float(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `double` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_double<W>(
        &self,
        tag: u32,
        val: &f64,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Bit64, dst)?;
        size += encode_double(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `double` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_double_list<W>(
        &self,
        tag: u32,
        vals: &Vec<f64>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_double(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `bytes` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_bytes<W>(
        &self,
        tag: u32,
        val: &Vec<u8>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::LengthDelimited, dst)?;
        size += encode_bytes(val.clone(), dst)?;
        Ok(size)
    }

    /// Encodes the provided `val` into `sint32` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sint32<W>(
        &self,
        tag: u32,
        val: &i32,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_sint32(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `sin32` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sint32_list<W>(
        &self,
        tag: u32,
        vals: &Vec<i32>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_sint32(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `sint64` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sint64<W>(
        &self,
        tag: u32,
        val: &i64,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Varint, dst)?;
        size += encode_sint64(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `sin64` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sint64_list<W>(
        &self,
        tag: u32,
        vals: &Vec<i64>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_sint64(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }
    
    /// Encodes the provided `val` into `fixed32` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_fixed32<W>(
        &self,
        tag: u32,
        val: &u32,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Bit32, dst)?;
        size += encode_fixed32(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `fixed32` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_fixed32_list<W>(
        &self,
        tag: u32,
        vals: &Vec<u32>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_fixed32(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `fixed64` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_fixed64<W>(
        &self,
        tag: u32,
        val: &u64,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Bit64, dst)?;
        size += encode_fixed64(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `fixed64` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_fixed64_list<W>(
        &self,
        tag: u32,
        vals: &Vec<u64>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_fixed64(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `sfixed32` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sfixed32<W>(
        &self,
        tag: u32,
        val: &i32,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Bit32, dst)?;
        size += encode_sfixed32(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `sfixed32` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sfixed32_list<W>(
        &self,
        tag: u32,
        vals: &Vec<i32>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_sfixed32(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }

    /// Encodes the provided `val` into `sfixed64` field with a specific `tag`
    /// number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sfixed64<W>(
        &self,
        tag: u32,
        val: &i64,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut size = 0;
        size += encode_key(tag, Typ::Bit64, dst)?;
        size += encode_sfixed64(*val, dst)?;
        Ok(size)
    }

    /// Encodes the provided `vals` into `sfixed64` repeated field with a
    /// specific `tag` number and writes the resulting bytes into `dst`.
    /// 
    /// On success the number of written bytes is returned otherwise an error is 
    /// thrown.
    pub fn encode_sfixed64_list<W>(
        &self,
        tag: u32,
        vals: &Vec<i64>,
        dst: &mut W,
    ) -> Result<usize, EncoderError>
    where
        W: ?Sized + io::Write,
    {
        let mut data = vec![];
        for val in vals {
            encode_sfixed64(*val, &mut data)?;
        }
        self.encode_bytes(tag, &data, dst)
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Should encode supported data types into Protocol Buffers buffer.
    #[test]
    fn encodes_supported() {
        let encoder = Encoder::default();
        let mut dst = vec![];
        let mut size = 0;
        size += encoder.encode((1, &b"foo".to_vec()), &mut dst).unwrap();
        size += encoder.encode((2, &true), &mut dst).unwrap();
        size += encoder.encode((3, &vec![false, true]), &mut dst).unwrap();
        size += encoder.encode((4, &1i32), &mut dst).unwrap();
        size += encoder.encode((5, &vec![-100i32, 100i32]), &mut dst).unwrap();
        size += encoder.encode((6, &1i64), &mut dst).unwrap();
        size += encoder.encode((7, &vec![-100i64, 100i64]), &mut dst).unwrap();
        size += encoder.encode((8, &1u32), &mut dst).unwrap();
        size += encoder.encode((9, &vec![1u32, 2u32]), &mut dst).unwrap();
        size += encoder.encode((10, &1u64), &mut dst).unwrap();
        size += encoder.encode((11, &vec![1u64, 2u64]), &mut dst).unwrap();
        size += encoder.encode((12, &1.0f32), &mut dst).unwrap();
        size += encoder.encode((13, &vec![1.0f32, 2.0f32]), &mut dst).unwrap();
        size += encoder.encode((14, &1.0f64), &mut dst).unwrap();
        size += encoder.encode((15, &vec![1.0f64, 2.0f64]), &mut dst).unwrap();
        size += encoder.encode((16, &b"foo".to_vec()), &mut dst).unwrap();
        size += encoder.encode((17, EncoderLit::SInt32(&-10)), &mut dst).unwrap();
        size += encoder.encode((18, EncoderLit::SInt32Vec(&vec![-10i32, 10i32])), &mut dst).unwrap();
        size += encoder.encode((19, EncoderLit::SInt64(&-10)), &mut dst).unwrap();
        size += encoder.encode((20, EncoderLit::SInt64Vec(&vec![-10i64, 10i64])), &mut dst).unwrap();
        size += encoder.encode((21, EncoderLit::Fixed32(&10)), &mut dst).unwrap();
        size += encoder.encode((22, EncoderLit::Fixed32Vec(&vec![1u32, 2u32])), &mut dst).unwrap();
        size += encoder.encode((23, EncoderLit::Fixed64(&10)), &mut dst).unwrap();
        size += encoder.encode((24, EncoderLit::Fixed64Vec(&vec![1u64, 2u64])), &mut dst).unwrap();
        size += encoder.encode((25, EncoderLit::SFixed32(&-10)), &mut dst).unwrap();
        size += encoder.encode((26, EncoderLit::SFixed32Vec(&vec![-10i32, 10i32])), &mut dst).unwrap();
        size += encoder.encode((27, EncoderLit::SFixed64(&-10)), &mut dst).unwrap();
        size += encoder.encode((28, EncoderLit::SFixed64Vec(&vec![-10i64, 10i64])), &mut dst).unwrap();
        assert_eq!(dst, vec![
            10, 3, 102, 111, 111,
            16, 1,
            26, 2, 0, 1,
            32, 1,
            42, 11, 156, 255, 255, 255, 255, 255, 255, 255, 255, 1, 100,
            48, 1,
            58, 11, 156, 255, 255, 255, 255, 255, 255, 255, 255, 1, 100,
            64, 1,
            74, 2, 1, 2,
            80, 1,
            90, 2, 1, 2,
            101, 0, 0, 128, 63,
            106, 8, 0, 0, 128, 63, 0, 0, 0, 64,
            113, 0, 0, 0, 0, 0, 0, 240, 63,
            122, 16, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 64,
            130, 1, 3, 102, 111, 111,
            136, 1, 19,
            146, 1, 2, 19, 20,
            152, 1, 19,
            162, 1, 2, 19, 20,
            173, 1, 10, 0, 0, 0,
            178, 1, 8, 1, 0, 0, 0, 2, 0, 0, 0,
            185, 1, 10, 0, 0, 0, 0, 0, 0, 0,
            194, 1, 16, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
            205, 1, 246, 255, 255, 255,
            210, 1, 8, 246, 255, 255, 255, 10, 0, 0, 0,
            217, 1, 246, 255, 255, 255, 255, 255, 255, 255,
            226, 1, 16, 246, 255, 255, 255, 255, 255, 255, 255, 10, 0, 0, 0, 0, 0, 0, 0,
        ]);
        assert_eq!(size, 209);
    }
}
