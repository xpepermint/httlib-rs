use std::io;
pub use super::EncoderError;
use crate::{Typ, Encoder};

/// Encodes the provided `val` into LEB128 format and writes the resulting
/// bytes into `buf`.
/// 
/// Varints are a method for serializing integers with one or more bytes. The
/// algorithm used here is known as [LEB128]. All bytes except the last have the
/// most significant bit (MSB) set (`C`), so that the decoder can determine
/// where the value ends. The other `7` bits (`N`) of each byte are intended to
/// represent the number.
/// 
/// LEB128 is an algorithm for encoding integers of arbitrary length in which
/// the bytes are arranged in a little-endian sequence. However, the Protocol
/// Buffers limit the size of the numbers to the supported data types.
/// 
/// ```txt
/// value = 150 (unsigned 32-bit)
/// 
/// Standard varint encoding:
///    XXXXXXXX 10010110 ... Number 150 in bytes.
///    X0000001 X0010110 ... Split to 7-bit sequence.
///    X0010110 X0000001 ... Revert the array of bytes.
///    10010110 00000001 ... Add MSB (1=continuation, 0=last byte).
/// ```
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_varint<W>(
    mut val: u64,
    buf: &mut W,
) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    let mut size = 0;
    loop {
        size += 1;
        if val < 0x80 {
            buf.write_all(&[val as u8 & 0x7F])?;
            return Ok(size);
        } else {
            let byte = ((val & 0x7F) | 0x80) as u8;
            buf.write_all(&[byte])?;
            val >>= 7;
        }
    }
}

/// Encodes field's key and writes the resulting bytes into `buf`.
/// 
/// The key is encoded as a `uint32` varint type, and in the last `3` bits (`T`)
/// contain the wire type. The key's field tag can thus be between `1` and
/// `2^29 - 1` = `536,870,911` (`0` is not a valid tag number).
/// 
/// ```txt
/// tag = 12345 (unsigned 32-bit), type = 1 (Varint)
/// 
/// 11001000 10000011 00000110 ... on the wire
/// CNNNNNNN CNNNNNNN CNNNNTTT ... bits per type
/// 
/// C = Contiunation, X = Number, T = Type
/// ```
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_key<W>(
    tag: u32,
    typ: Typ,
    buf: &mut W,
) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    if tag < Encoder::TAG_MIN || tag > Encoder::TAG_MAX {
        return Err(EncoderError::InvalidTag);
    }

    let key = (tag << 3) | typ as u32;
    encode_varint(u64::from(key), buf)
}

/// Encodes the provided `val` into `bool` format and writes the resulting
/// bytes into `buf`.
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_bool<W>(val: bool, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    let res = if val { 1u64 } else { 0u64 };
    encode_varint(res, buf)
}

/// Encodes the provided `val` into `int32` format and writes the resulting
/// bytes into `buf`.
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_int32<W>(val: i32, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    encode_varint(val as u64, buf)
}

/// Encodes the provided `val` into `int64` format and writes the resulting
/// bytes into `buf`.
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_int64<W>(val: i64, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    encode_varint(val as u64, buf)
}

/// Encodes the provided `val` into `uint32` format and writes the resulting
/// bytes into `buf`.
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_uint32<W>(val: u32, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    encode_varint(val as u64, buf)
}

/// Encodes the provided `val` into `uint64` format and writes the resulting
/// bytes into `buf`.
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_uint64<W>(val: u64, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    encode_varint(val, buf)
}

/// Encodes the provided `val` into `sint32` format and writes the resulting
/// bytes into `buf`.
/// 
/// There is a big difference between signed integer types (`sint32`) and the
/// "standard" integer types (`int32`). If you use `int32` as the type for a
/// negative number, the result is always ten bytes long, which makes a very
/// large unsigned integer. In case you know that the value will most likely be
/// negative, you can optimize the result and use one of the signed types, where
/// the resulting varint uses [ZigZag] encoding for efficiency. Essentially,
/// this means that the positive and negative integers are zigzagged through, so
/// that `-1` is encoded as `1`, `1` as `2`, `-2` as `3`, and so on.
/// 
/// ```txt
/// value = -12345 (signed 32-bit)
/// 
/// Signed 32-bit varint encoding:
///    -12345 ... Unsigned 32-bit integer.
///     24689 ... ZigZag value using (value << 1) ^ (value >> 31).
///           ... Continue with the standard varint encoding.
/// ```
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_sint32<W>(val: i32, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    let res = ((val << 1) ^ (val >> 31)) as u32 as u64;
    encode_varint(res, buf)
}

/// Encodes the provided `val` into `sint64` format and writes the resulting
/// bytes into `buf`.
/// 
/// There is a big difference between signed integer types (`sint64`) and the
/// "standard" integer types (`int64`). If you use `int64` as the type for a
/// negative number, the result is always ten bytes long, which makes a very
/// large unsigned integer. In case you know that the value will most likely be
/// negative, you can optimize the result and use one of the signed types, where
/// the resulting varint uses [ZigZag] encoding for efficiency. Essentially,
/// this means that the positive and negative integers are zigzagged through, so
/// that `-1` is encoded as `1`, `1` as `2`, `-2` as `3`, and so on.
/// 
/// ```txt
/// value = -54321 (signed 64-bit)
/// 
/// Signed 64-bit varint encoding:
///    -54321 ... Unsigned 64-bit integer.
///    108641 ... ZigZag value using (val << 1) ^ (val >> 63).
///           ... Continue with the standard varint encoding.
/// ```
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_sint64<W>(val: i64, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    let res = ((val << 1) ^ (val >> 63)) as u64;
    encode_varint(res, buf)
}

/// Encodes the provided `val` into `fixed32` format and writes the resulting
/// bytes into `buf`.
/// 
/// Such format represents an encoded 32-bit number with wire types `5`. Fixed
/// size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// ```txt
/// value = 12345 (signed 32-bit)
/// 
/// Fixed size encoding:
///    00000000 00000000 00110000 00111001 ... Value in (big-endian) bytes.
///    00111001 00110000 00000000 00000000 ... Reverse bytes to little-endian order.
/// ```
/// 
/// Use this format only when data are predictible and you know that the result
/// will be smaller then when using the "standard" formats. 
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_fixed32<W>(val: u32, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    buf.write_all(&val.to_le_bytes())?;
    Ok(4)
}

/// Encodes the provided `val` into `fixed64` format and writes the resulting
/// bytes into `buf`.
/// 
/// Such format represents an encoded 64-bit number with wire types `1`. Fixed
/// size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// Use this format only when data are predictible and you know that the result
/// will be smaller then when using the "standard" formats. 
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_fixed64<W>(val: u64, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    buf.write_all(&val.to_le_bytes())?;
    Ok(8)
}

/// Encodes the provided `val` into `sfixed32` format and writes the resulting
/// bytes into `buf`.
/// 
/// Such format represents an encoded 32-bit number with wire types `5`. Fixed
/// size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// Use this format only when data are predictible and you know that the result
/// will be smaller then when using the "standard" formats. 
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_sfixed32<W>(val: i32, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    buf.write_all(&val.to_le_bytes())?;
    Ok(4)
}

/// Encodes the provided `val` into `sfixed64` format and writes the resulting
/// bytes into `buf`.
/// 
/// Such format represents an encoded 64-bit number with wire types `1`. Fixed
/// size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// Use this format only when data are predictible and you know that the result
/// will be smaller then when using the "standard" formats. 
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_sfixed64<W>(val: i64, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    buf.write_all(&val.to_le_bytes())?;
    Ok(8)
}

/// Encodes the provided `val` into `float` format and writes the resulting
/// bytes into `buf`.
/// 
/// Float is encoded as a 32-bit number with wire types `5`. Fixed size number
/// format is represented by bytes in little-endian byte order (reversed order).
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_float<W>(val: f32, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    buf.write_all(&val.to_le_bytes())?;
    Ok(4)
}

/// Encodes the provided `val` into `double` format and writes the resulting
/// bytes into `buf`.
/// 
/// Double is encoded as a 64-bit number with wire types `1`. Fixed size number
/// format is represented by bytes in little-endian byte order (reversed order).
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_double<W>(val: f64, buf: &mut W) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    buf.write_all(&val.to_le_bytes())?;
    Ok(8)
}

/// Wraps the provided `val` into `bytes` format and writes the resulting bytes
/// into `buf`.
/// 
/// Length-delimited type, represented with number `2`, encodes data into a
/// sequence of bytes prepended with a value of varint encoded which represents
/// the number of bytes that represent the content. This describes data types
/// `bytes` and `string`, `embedded` messages (nested objects) and `repated`
/// numeric fields.
/// 
/// ```txt
/// value = b"foo"
/// 
/// Length-delimited encoding:
///    00000011 XXXXXXXX XXXXXXXX XXXXXXXX ... Encode message size (3 bytes) as standard 32-bit varint.
///    00000011 01100110 01101111 01101111 ... Append string (foo) in bytes.
/// ```
/// 
/// On success the number of written bytes is returned otherwise en error is 
/// thrown.
pub fn encode_bytes<W>(
    mut bytes: Vec<u8>,
    buf: &mut W,
) -> Result<usize, EncoderError>
where
    W: ?Sized + io::Write,
{
    let mut size = bytes.len();
    if size > u64::MAX as usize {
        return Err(EncoderError::DataOverflow);
    }
    size += encode_varint(size as u64, buf)?; // number of bytes
    buf.write_all(&mut bytes)?;
    Ok(size)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Should encode a number into LEB128 wire format.
    #[test]
    fn encodes_varints() {
        let mut dst = vec![];
        let size = encode_varint(u64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0]);
        assert_eq!(size, 1);

        dst.clear();
        let size = encode_varint(u64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
        assert_eq!(size, 10);
    }

    /// Should encode field's header key which consists of field tag number and
    /// field wire type.
    #[test]
    fn encodes_key() {
        let mut dst = vec![];
        let size = encode_key(123456789, Typ::Varint, &mut dst).unwrap();
        assert_eq!(dst, vec![168, 209, 249, 214, 3]);
        assert_eq!(size, 5);

        dst.clear();
        let size = encode_key(1234, Typ::Bit32, &mut dst).unwrap();
        assert_eq!(dst, vec![149, 77]);
        assert_eq!(size, 2);
    }

    /// Should encode a numeric value as `int32` data type.
    #[test]
    fn encodes_int32() {
        let mut dst = vec![];
        let size = encode_int32(i32::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1]);
        assert_eq!(size, 10);

        dst.clear();
        let size = encode_int32(i32::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 7]);
        assert_eq!(size, 5);
    }

    /// Should encode a numeric value as `uint32` data type.
    #[test]
    fn encodes_uint32() {
        let mut dst = vec![];
        let size = encode_uint32(u32::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0]);
        assert_eq!(size, 1);

        dst.clear();
        let size = encode_uint32(u32::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 15]);
        assert_eq!(size, 5);
    }

    /// Should encode a numeric value as `sint32` data type.
    #[test]
    fn encodes_sint32() {
        let mut dst = vec![];
        let size = encode_sint32(i32::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 15]);
        assert_eq!(size, 5);

        dst.clear();
        let size = encode_sint32(i32::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![254, 255, 255, 255, 15]);
        assert_eq!(size, 5);
    }

    /// Should encode a numeric value as `int64` data type.
    #[test]
    fn encodes_int64() {
        let mut dst = vec![];
        let size = encode_int64(i64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
        assert_eq!(size, 10);

        dst.clear();
        let size = encode_int64(i64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 255, 255, 127]);
        assert_eq!(size, 9);
    }

    /// Should encode a numeric value as `uint64` data type.
    #[test]
    fn encodes_uint64() {
        let mut dst = vec![];
        let size = encode_uint64(u64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0]);
        assert_eq!(size, 1);

        dst.clear();
        let size = encode_uint64(u64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
        assert_eq!(size, 10);
    }

    /// Should encode a numeric value as `sint64` data type.
    #[test]
    fn encodes_sint64() {
        let mut dst = vec![];
        let size = encode_sint64(i64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
        assert_eq!(size, 10);

        dst.clear();
        let size = encode_sint64(i64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![254, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
        assert_eq!(size, 10);
    }

    /// Should encode a numeric value as `bool` data type.
    #[test]
    fn encodes_bool() {
        let mut dst = vec![];
        let size = encode_bool(true, &mut dst).unwrap();
        assert_eq!(dst, vec![1]);
        assert_eq!(size, 1);

        dst.clear();
        let size = encode_bool(false, &mut dst).unwrap();
        assert_eq!(dst, vec![0]);
        assert_eq!(size, 1);
    }

    /// Should encode a numeric value as `fixed64` data type.
    #[test]
    fn encodes_fixed64() {
        let mut dst = vec![];
        let size = encode_fixed64(u64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(size, 8);

        dst.clear();
        let size = encode_fixed64(u64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(size, 8);
    }

    /// Should encode a numeric value as `sfixed64` data type.
    #[test]
    fn encodes_sfixed64() {
        let mut dst = vec![];
        let size = encode_sfixed64(i64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0, 0, 0, 0, 0, 0, 0, 128]);
        assert_eq!(size, 8);

        dst.clear();
        let size = encode_sfixed64(i64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 255, 127]);
        assert_eq!(size, 8);
    }

    /// Should encode a numeric value as `double` data type.
    #[test]
    fn encodes_double() {
        let mut dst = vec![];
        let size = encode_double(f64::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 239, 255]);
        assert_eq!(size, 8);

        dst.clear();
        let size = encode_double(f64::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255, 255, 255, 239, 127]);
        assert_eq!(size, 8);
    }

    /// Should encode a numeric value as `fixed32` data type.
    #[test]
    fn encodes_fixed32() {
        let mut dst = vec![];
        let size = encode_fixed32(u32::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0, 0, 0, 0]);
        assert_eq!(size, 4);

        dst.clear();
        let size = encode_fixed32(u32::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 255]);
        assert_eq!(size, 4);
    }

    /// Should encode a numeric value as `sfixed32` data type.
    #[test]
    fn encodes_sfixed32() {
        let mut dst = vec![];
        let size = encode_sfixed32(i32::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![0, 0, 0, 128]);
        assert_eq!(size, 4);

        dst.clear();
        let size = encode_sfixed32(i32::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 255, 127]);
        assert_eq!(size, 4);
    }

    /// Should encode a numeric value as `float` data type.
    #[test]
    fn encodes_float() {
        let mut dst = vec![];
        let size = encode_float(f32::MIN, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 127, 255]);
        assert_eq!(size, 4);

        dst.clear();
        let size = encode_float(f32::MAX, &mut dst).unwrap();
        assert_eq!(dst, vec![255, 255, 127, 127]);
        assert_eq!(size, 4);
    }

    /// Should encode a bytes value as raw `bytes` data type.
    #[test]
    fn encodes_bytes() {
        let mut dst = vec![];
        let size = encode_bytes(vec![1, 2, 3, 4, 5], &mut dst).unwrap();
        assert_eq!(dst, vec![5, 1, 2, 3, 4, 5]);
        assert_eq!(size, 6);
    }
}
