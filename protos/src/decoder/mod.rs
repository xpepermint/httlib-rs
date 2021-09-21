//! Provides an implementation of the `proto3` decoder.
//! 
//! The decoder performs the task of translating encoded binary data into actual
//! data fields.

mod error;
mod from_lit;
mod lit;
mod primitives;

use crate::Typ;
pub use error::*;
pub use from_lit::*;
pub use lit::*;
use primitives::*;

/// Provides the decoding engine for Protocol Buffers.
pub struct Decoder {
    /// A key with tag number and wire type of the currently decoding field.
    key: (u32, Typ),

    /// The number of bytes that needs to be read for the currently decoding
    /// length-delimited field.
    ld_len: Option<u64>,
}

impl Decoder {
    /// Decodes `proto3` encoded fields from the provided `buf` and writes the
    /// result into `dst`.
    /// 
    /// The returned fields are tuples of format `(tag, type, bytes)` where the
    /// returned bytes represent the encoded value. The developer should wrap
    /// each value into the desired `DecoderLit` and call `parse` on it. 
    /// 
    /// ```rust
    /// use httlib_protos::{Decoder, DecoderLit};
    /// 
    /// let mut decoder = Decoder::default();
    /// 
    /// let mut buf = vec![0x85, 0x35, 0x85];
    /// 
    /// let mut dst = vec![];
    /// let size = decoder.decode(&mut buf, &mut dst).unwrap();
    /// 
    /// for (tag, typ, byt)  in dst {
    ///     if tag == 1 {
    ///         DecoderLit::Int32(byt).parse::<i32>().unwrap();
    ///     }
    /// }
    /// ```
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    pub fn decode(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(u32, Typ, Vec<u8>)>,
    ) -> Result<usize, DecoderError> {
        let mut total = 0;
        loop {
            let mut _size = 0;
            match self.key.1 {
                Typ::Unknown => {
                    _size = self.decode_key(buf)?;
                },
                Typ::Varint => {
                    _size = self.extract_varint(buf, dst)?;
                },
                Typ::Bit32 => {
                    _size = self.extract_bit32(buf, dst)?;
                },
                Typ::Bit64 => {
                    _size = self.extract_bit64(buf, dst)?;
                },
                Typ::LengthDelimited => {
                    _size = self.extract_ld(buf, dst)?;
                },
            }
            if _size == 0 {
                break;
            }
            total += _size;
        }
        Ok(total)
    }

    /// Decodes an encoded field key from the provided `buf`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn decode_key(&mut self, buf: &mut Vec<u8>) -> Result<usize, DecoderError> {
        let size = match decode_key(&buf, &mut self.key) {
            Ok(size) => size,
            Err(DecoderError::InputUnderflow) => return Ok(0),
            Err(e) => return Err(e),
        };
        buf.drain(..size);
        Ok(size)
    }

    /// Reads bytes for value with wire type `0` from the provided `buf` and
    /// writes the resulting bytes into `dst`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn extract_varint(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(u32, Typ, Vec<u8>)>,
    ) -> Result<usize, DecoderError> {
        let mut bytes = vec![];
        let size = match extract_varint(&buf, &mut bytes) {
            Ok(size) => size,
            Err(DecoderError::InputUnderflow) => return Ok(0),
            Err(e) => return Err(e),
        };
        dst.push((self.key.0, self.key.1, bytes));
        buf.drain(..size);
        self.reset();
        Ok(size)
    }

    /// Reads bytes for value with wire type `5` from the provided `buf` and
    /// writes the resulting bytes into `dst`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn extract_bit32(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(u32, Typ, Vec<u8>)>,
    ) -> Result<usize, DecoderError> {
        let mut bytes = vec![];
        let size = match extract_bit32(&buf, &mut bytes) {
            Ok(size) => size,
            Err(DecoderError::InputUnderflow) => return Ok(0),
            Err(e) => return Err(e),
        };
        dst.push((self.key.0, self.key.1, bytes));
        buf.drain(..size);
        self.reset();
        Ok(size)
    }

    /// Reads bytes for value with wire type `1` from the provided `buf` and
    /// writes the resulting bytes into `dst`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn extract_bit64(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(u32, Typ, Vec<u8>)>,
    ) -> Result<usize, DecoderError> {
        let mut bytes = vec![];
        let size = match extract_bit64(&buf, &mut bytes) {
            Ok(size) => size,
            Err(DecoderError::InputUnderflow) => return Ok(0),
            Err(e) => return Err(e),
        };
        dst.push((self.key.0, self.key.1, bytes));
        buf.drain(..size);
        self.reset();
        Ok(size)
    }

    /// Reads bytes for value with wire type `2` from the provided `buf` and
    /// writes the resulting bytes into `dst`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn extract_ld(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(u32, Typ, Vec<u8>)>,
    ) -> Result<usize, DecoderError> {
        if self.ld_len.is_some() {
            self.extract_ld_bytes(buf, dst)
        } else {
            self.decode_ld_len(buf)
        }
    }

    /// Decodes an encoded length of the currently handled length-delimited
    /// field from the provided `buf`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn decode_ld_len(
        &mut self,
        buf: &mut Vec<u8>,
    ) -> Result<usize, DecoderError> {
        let mut val = 0;
        let size = match decode_varint(&buf, &mut val) {
            Ok(size) => size,
            Err(DecoderError::InputUnderflow) => return Ok(0),
            Err(e) => return Err(e),
        };
        self.ld_len = Some(val);
        buf.drain(..size);
        Ok(size)
    }

    /// Reads bytes of the currently handled length-delimited field from the
    /// provided `buf` and writes the resulting bytes into `dst`.
    /// 
    /// This function consumes the buffer only if the decoding succeeds. The
    /// provided vector will stay untouched in case of an error or insuficioent
    /// data.
    /// 
    /// On success the number of written bytes is returned otherwise en error is 
    /// thrown.
    fn extract_ld_bytes(
        &mut self,
        buf: &mut Vec<u8>,
        dst: &mut Vec<(u32, Typ, Vec<u8>)>,
    ) -> Result<usize, DecoderError> {
        let len = self.ld_len.unwrap();
        let mut bytes = vec![];
        let size = match extract_ld(&buf, len, &mut bytes) {
            Ok(size) => size,
            Err(DecoderError::InputUnderflow) => return Ok(0),
            Err(e) => return Err(e),
        };
        dst.push((self.key.0, self.key.1, bytes));
        buf.drain(..size);
        self.reset();
        Ok(size)
    }

    /// Resets the decoder and flushes all memoried data.
    fn reset(&mut self) {
        self.key = (0, Typ::Unknown);
        self.ld_len = None;
    }
}

impl<'a> Default for Decoder {
    fn default() -> Self {
        Self {
            key: (0, Typ::Unknown),
            ld_len: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Should decode supported formats from Protocol Buffers bytes.
    #[test]
    fn decodes_supported() {
        let mut decoder = Decoder::default();
        let mut dst = vec![];
        let mut src = vec![
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
        ];
        let size = decoder.decode(&mut src, &mut dst).unwrap(); // decode supported fields
        let mut index = 0;
        for (tag, typ, byt) in dst {
            index += 1;
            assert_eq!(tag, index);
            if index == 1 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(byt, vec![102, 111, 111]);
                assert_eq!(DecoderLit::Bytes(byt).parse::<Vec<u8>>(), Ok(vec![102, 111, 111]));
            } else if index == 2 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::Bool(byt).parse::<bool>(), Ok(true));
            } else if index == 3 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::BoolList(byt).parse::<Vec<bool>>(), Ok(vec![false, true]));
            } else if index == 4 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::Int32(byt).parse::<i32>(), Ok(1));
            } else if index == 5 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::Int32List(byt).parse::<Vec<i32>>(), Ok(vec![-100i32, 100i32]));
            } else if index == 6 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::Int64(byt).parse::<i64>(), Ok(1i64));
            } else if index == 7 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::Int64List(byt).parse::<Vec<i64>>(), Ok(vec![-100i64, 100i64]));
            } else if index == 8 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::UInt32(byt).parse::<u32>(), Ok(1u32));
            } else if index == 9 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::UInt32List(byt).parse::<Vec<u32>>(), Ok(vec![1u32, 2u32]));
            } else if index == 10 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::UInt64(byt).parse::<u64>(), Ok(1u64));
            } else if index == 11 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::UInt64List(byt).parse::<Vec<u64>>(), Ok(vec![1u64, 2u64]));
            } else if index == 12 {
                assert_eq!(typ, Typ::Bit32);
                assert_eq!(DecoderLit::Float(byt).parse::<f32>(), Ok(1.0f32));
            } else if index == 13 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::FloatList(byt).parse::<Vec<f32>>(), Ok(vec![1.0f32, 2.0f32]));
            } else if index == 14 {
                assert_eq!(typ, Typ::Bit64);
                assert_eq!(DecoderLit::Double(byt).parse::<f64>(), Ok(1.0f64));
            } else if index == 15 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::DoubleList(byt).parse::<Vec<f64>>(), Ok(vec![1.0f64, 2.0f64]));
            } else if index == 16 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::Bytes(byt).parse::<String>(), Ok(String::from("foo")));
            } else if index == 17 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::SInt32(byt).parse::<i32>(), Ok(-10));
            } else if index == 18 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::SInt32List(byt).parse::<Vec<i32>>(), Ok(vec![-10i32, 10i32]));
            } else if index == 19 {
                assert_eq!(typ, Typ::Varint);
                assert_eq!(DecoderLit::SInt64(byt).parse::<i64>(), Ok(-10));
            } else if index == 20 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::SInt64List(byt).parse::<Vec<i64>>(), Ok(vec![-10i64, 10i64]));
            } else if index == 21 {
                assert_eq!(typ, Typ::Bit32);
                assert_eq!(DecoderLit::Fixed32(byt).parse::<u32>(), Ok(10));
            } else if index == 22 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::Fixed32List(byt).parse::<Vec<u32>>(), Ok(vec![1u32, 2u32]));
            } else if index == 23 {
                assert_eq!(typ, Typ::Bit64);
                assert_eq!(DecoderLit::Fixed64(byt).parse::<u64>(), Ok(10));
            } else if index == 24 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::Fixed64List(byt).parse::<Vec<u64>>(), Ok(vec![1u64, 2u64]));
            } else if index == 25 {
                assert_eq!(typ, Typ::Bit32);
                assert_eq!(DecoderLit::SFixed32(byt).parse::<i32>(), Ok(-10));
            } else if index == 26 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::SFixed32List(byt).parse::<Vec<i32>>(), Ok(vec![-10i32, 10i32]));
            } else if index == 27 {
                assert_eq!(typ, Typ::Bit64);
                assert_eq!(DecoderLit::SFixed64(byt).parse::<i64>(), Ok(-10));
            } else if index == 28 {
                assert_eq!(typ, Typ::LengthDelimited);
                assert_eq!(DecoderLit::SFixed64List(byt).parse::<Vec<i64>>(), Ok(vec![-10i64, 10i64]));
            }
        }
        assert_eq!(size, 209); // read bytes
        let mut dst = vec![];
        let mut src = vec![0];
        assert!(decoder.decode(&mut src, &mut dst).is_err()); // handles errors
    }
}
