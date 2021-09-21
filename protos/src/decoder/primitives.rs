use core::convert::TryFrom;
pub use super::DecoderError;
use crate::{Typ, Encoder};

/// Decodes a LEB128 encoded number from the provided `buf` and writes the
/// resulting bytes into `dst`.
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
/// Standard varint decoding:
///    10010110 00000001 ... Encoded number.
///    00000001 10010110 ... Revert the array of bytes.
///    X0000001 X0010110 ... Remove MSB.
///    XXXXXXXX 10010110 ... Merge bits together (number 150 in bytes).
/// ```
/// 
/// On success, the number of read bytes is returned otherwise an error is
/// thrown.
pub fn decode_varint(buf: &[u8], dst: &mut u64) -> Result<usize, DecoderError> {
    let mut value: u64 = 0;
    let mut count = 0;
    loop {
        let byte = match buf.get(count) {
            Some(b) => *b,
            None => return Err(DecoderError::InputUnderflow),
        };
        value |= u64::from(byte & 0x7F) << (7 * count);
        count += 1;
        if byte <= 0x7F {
            *dst = value;
            return Ok(count);
        }
    }
}

/// Decodes an encoded field key from the provided `buf` and writes the
/// resulting bytes into `dst`.
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
/// On success, the number of read bytes is returned otherwise an error is
/// thrown.
pub fn decode_key(
    buf: &[u8],
    dst: &mut (u32, Typ),
) -> Result<usize, DecoderError> {
    let mut key = 0;
    let size = decode_varint(buf, &mut key)?;
    if key > u64::from(u32::MAX) {
        return Err(DecoderError::InvalidInput);
    }

    let typ = Typ::try_from(key & 0x07)?;
    let tag = key as u32 >> 3;
    if tag < Encoder::TAG_MIN {
        return Err(DecoderError::InvalidInput);
    } else if typ == Typ::Unknown {
        return Err(DecoderError::InvalidInput);
    }

    *dst = (tag, typ);
    Ok(size)
}

/// Decodes an encoded `bool` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_bool(buf: &[u8], dst: &mut bool) -> Result<usize, DecoderError> {
    let mut val = 0;
    let size = decode_varint(buf, &mut val)?;
    *dst = if val == 0u64 { false } else { true };
    Ok(size)
}

/// Decodes a length-delimited encoded repeated `bool` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_bool_list(
    buf: &[u8],
    dst: &mut Vec<bool>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size  || total == 0 {
        let mut val = false;
        size += decode_bool(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `int32` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_int32(buf: &[u8], dst: &mut i32) -> Result<usize, DecoderError> {
    let mut val = 0;
    let size = decode_varint(buf, &mut val)?;
    *dst = val as i32;
    Ok(size)
}

/// Decodes a length-delimited encoded repeated `int32` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_int32_list(
    buf: &[u8],
    dst: &mut Vec<i32>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_int32(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `int64` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_int64(buf: &[u8], dst: &mut i64) -> Result<usize, DecoderError> {
    let mut val = 0;
    let size = decode_varint(buf, &mut val)?;
    *dst = val as i64;
    Ok(size)
}

/// Decodes a length-delimited encoded repeated `int64` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_int64_list(
    buf: &[u8],
    dst: &mut Vec<i64>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_int64(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `uint32` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_uint32(buf: &[u8], dst: &mut u32) -> Result<usize, DecoderError> {
    let mut val = 0;
    let size = decode_varint(buf, &mut val)?;
    *dst = val as u32;
    Ok(size)
}

/// Decodes a length-delimited encoded repeated `uint32` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_uint32_list(
    buf: &[u8],
    dst: &mut Vec<u32>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_uint32(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `uint64` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_uint64(buf: &[u8], dst: &mut u64) -> Result<usize, DecoderError> {
    decode_varint(buf, dst)
}

/// Decodes a length-delimited encoded repeated `uint64` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_uint64_list(
    buf: &[u8],
    dst: &mut Vec<u64>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_varint(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `sint32` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
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
/// Signed 32-bit varint decoding:
///           ... Start with the standard varint decoding.
///     24689 ... ZigZag value using (value >> 1) ^ -(value & 1).
///    -12345 ... Unsigned 32-bit integer.
/// ```
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sint32(buf: &[u8], dst: &mut i32) -> Result<usize, DecoderError> {
    let mut val = 0;
    let size = decode_varint(buf, &mut val)?;
    *dst = ((val >> 1) as i32) ^ (-((val & 1) as i32));
    Ok(size)
}

/// Decodes a length-delimited encoded repeated `sint32` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sint32_list(
    buf: &[u8],
    dst: &mut Vec<i32>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_sint32(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `sint64` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
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
/// Signed 64-bit varint decoding:
///           ... Start with the standard varint decoding.
///    108641 ... ZigZag value using (value >> 1) ^ -(value & 1).
///    -54321 ... Unsigned 64-bit integer.
/// ```
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sint64(buf: &[u8], dst: &mut i64) -> Result<usize, DecoderError> {
    let mut val = 0;
    let size = decode_varint(buf, &mut val)?;
    *dst = ((val >> 1) as i64) ^ (-((val & 1) as i64));
    Ok(size)
}

/// Decodes a length-delimited encoded repeated `sint64` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sint64_list(
    buf: &[u8],
    dst: &mut Vec<i64>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_sint64(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `fixed32` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// Such format represents an encoded 32-bit number with wire types `5`.
/// Fixed-size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// ```txt
/// value = 12345 (signed 32-bit)
/// 
/// Fixed-size decoding:
///    00111001 00110000 00000000 00000000 ... Encoded value in (little-endian) order.
///    00000000 00000000 00110000 00111001 ... Value in (big-endian) bytes.
/// ```
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_fixed32(
    buf: &[u8],
    dst: &mut u32,
) -> Result<usize, DecoderError> {
    if buf.len() < 4 {
        return Err(DecoderError::InputUnderflow)
    }
    let mut bytes = [0u8; 4];
    bytes.clone_from_slice(&buf[0..4]);
    *dst = u32::from_le_bytes(bytes);
    Ok(4)
}

/// Decodes a length-delimited encoded repeated `fixed32` value from the
/// provided `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_fixed32_list(
    buf: &[u8],
    dst: &mut Vec<u32>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_fixed32(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `fixed64` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// Such format represents an encoded 64-bit number with wire types `1`.
/// Fixed-size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_fixed64(
    buf: &[u8],
    dst: &mut u64,
) -> Result<usize, DecoderError> {
    if buf.len() < 8 {
        return Err(DecoderError::InputUnderflow)
    }
    let mut bytes = [0u8; 8];
    bytes.clone_from_slice(&buf[0..8]);
    *dst = u64::from_le_bytes(bytes);
    Ok(8)
}

/// Decodes a length-delimited encoded repeated `fixed64` value from the
/// provided `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_fixed64_list(
    buf: &[u8],
    dst: &mut Vec<u64>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_fixed64(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `sfixed32` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// Such format represents an encoded 32-bit number with wire types `5`.
/// Fixed-size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sfixed32(
    buf: &[u8],
    dst: &mut i32,
) -> Result<usize, DecoderError> {
    if buf.len() < 4 {
        return Err(DecoderError::InputUnderflow)
    }
    let mut bytes = [0u8; 4];
    bytes.clone_from_slice(&buf[0..4]);
    *dst = i32::from_le_bytes(bytes);
    Ok(4)
}

/// Decodes a length-delimited encoded repeated `sfixed32` value from the
/// provided `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sfixed32_list(
    buf: &[u8],
    dst: &mut Vec<i32>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_sfixed32(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `sfixed64` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// Such format represents an encoded 64-bit number with wire types `1`.
/// Fixed-size number format is represented by bytes in little-endian byte order
/// (reversed order).
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sfixed64(
    buf: &[u8],
    dst: &mut i64,
) -> Result<usize, DecoderError> {
    if buf.len() < 8 {
        return Err(DecoderError::InputUnderflow)
    }
    let mut bytes = [0u8; 8];
    bytes.clone_from_slice(&buf[0..8]);
    *dst = i64::from_le_bytes(bytes);
    Ok(8)
}

/// Decodes a length-delimited encoded repeated `sfixed64` value from the
/// provided `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_sfixed64_list(
    buf: &[u8],
    dst: &mut Vec<i64>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0;
        size += decode_sfixed64(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `float` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// Float is encoded as a 32-bit number with wire types `5`. Fixed-size number
/// format is represented by bytes in little-endian byte order (reversed order).
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_float(buf: &[u8], dst: &mut f32) -> Result<usize, DecoderError> {
    if buf.len() < 4 {
        return Err(DecoderError::InputUnderflow)
    }
    let mut bytes = [0u8; 4];
    bytes.clone_from_slice(&buf[0..4]);
    *dst = f32::from_le_bytes(bytes);
    Ok(4)
}

/// Decodes a length-delimited encoded repeated `float` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_float_list(
    buf: &[u8],
    dst: &mut Vec<f32>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0.0;
        size += decode_float(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Decodes an encoded `double` value from the provided `buf` and writes the
/// resulting bytes into `dst`.
/// 
/// Double is encoded as a 64-bit number with wire types `1`. Fixed-size number
/// format is represented by bytes in little-endian byte order (reversed order).
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_double(buf: &[u8], dst: &mut f64) -> Result<usize, DecoderError> {
    if buf.len() < 8 {
        return Err(DecoderError::InputUnderflow)
    }
    let mut bytes = [0u8; 8];
    bytes.clone_from_slice(&buf[0..8]);
    *dst = f64::from_le_bytes(bytes);
    Ok(8)
}

/// Decodes a length-delimited encoded repeated `double` value from the provided
/// `buf` and writes the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn decode_double_list(
    buf: &[u8],
    dst: &mut Vec<f64>,
) -> Result<usize, DecoderError> {
    let total = buf.len();
    let mut size = 0;
    let mut items = vec![];
    while total > size || total == 0 {
        let mut val = 0.0;
        size += decode_double(&buf[size..], &mut val)?;
        items.push(val);
    }
    dst.append(&mut items);
    Ok(size)
}

/// Reads bytes for value with wire type `0` from the provided `but` and writes
/// the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn extract_varint(
    buf: &[u8],
    dst: &mut Vec<u8>,
) -> Result<usize, DecoderError> {
    let mut bytes = vec![];
    let mut count = 0;
    loop {
        let byte = match buf.get(count) {
            Some(b) => *b,
            None => return Err(DecoderError::InputUnderflow),
        };
        bytes.push(byte.clone());
        count += 1;
        if byte <= 0x7F {
            dst.append(&mut bytes);
            return Ok(count);
        }
    }
}

/// Reads bytes for value with wire type `5` from the provided `but` and writes
/// the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn extract_bit32(
    buf: &[u8],
    dst: &mut Vec<u8>,
) -> Result<usize, DecoderError> {
    if buf.len() < 4 {
        return Err(DecoderError::InputUnderflow);
    }
    dst.append(&mut buf[0..4].to_vec());
    Ok(4)
}

/// Reads bytes for value with wire type `1` from the provided `but` and writes
/// the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn extract_bit64(
    buf: &[u8],
    dst: &mut Vec<u8>,
) -> Result<usize, DecoderError> {
    if buf.len() < 8 {
        return Err(DecoderError::InputUnderflow);
    }
    dst.append(&mut buf[0..8].to_vec());
    Ok(8)
}

/// Reads bytes for value with wire type `2` from the provided `but` and writes
/// the resulting bytes into `dst`.
/// 
/// On success, the number of read bytes is returned otherwise an error is 
/// thrown.
pub fn extract_ld(
    buf: &[u8],
    size: u64,
    dst: &mut Vec<u8>,
) -> Result<usize, DecoderError> {
    let size = size as usize;
    if buf.len() < size {
        return Err(DecoderError::InputUnderflow);
    }
    dst.append(&mut buf[0..size].to_vec());
    Ok(size)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Should decode a LEB128 encoded stream of values.
    #[test]
    fn decodes_varints() {
        let mut dst = 0;
        let buf = vec![0xb9];
        assert!(decode_varint(&buf, &mut dst).is_err());
        let buf = vec![0xb9, 0x60, 0x85];
        let size = decode_varint(&buf, &mut dst).unwrap();
        assert_eq!(dst, 12345);
        assert_eq!(size, 2);
        let buf = vec![0x85, 0x35, 0x85];
        let size = decode_varint(&buf, &mut dst).unwrap();
        assert_eq!(dst, 6789);
        assert_eq!(size, 2);
    }

    /// Should decode a header key of a field which consists of field tag number
    /// and field wire type.
    #[test]
    fn decodes_key() {
        let mut dst = (0, Typ::Unknown);
        let buf = vec![0xcd, 0x83, 0x06, 0xaa];
        let size = decode_key(&buf, &mut dst).unwrap();
        assert_eq!(dst.0, 12345);
        assert_eq!(dst.1, Typ::Bit32);
        assert_eq!(size, 3);
        let buf = vec![0xaa, 0xa8, 0x03, 0x03];
        let size = decode_key(&buf, &mut dst).unwrap();
        assert_eq!(dst.0, 6789);
        assert_eq!(dst.1, Typ::LengthDelimited);
        assert_eq!(size, 3);
    }

    /// Should decode an encoded numeric value into `bool` data type.
    #[test]
    fn decodes_bool() {
        let mut dst = true;
        let buf = vec![0x00];
        let size = decode_bool(&buf, &mut dst).unwrap();
        assert_eq!(dst, false);
        assert_eq!(size, 1);
        let buf = vec![0x01];
        let size = decode_bool(&buf, &mut dst).unwrap();
        assert_eq!(dst, true);
        assert_eq!(size, 1);
    }

    /// Should decode an encoded numeric value into `int32` data type.
    #[test]
    fn decodes_int32() {
        let mut dst = 0;
        let buf = vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01, 0xff];
        let size = decode_int32(&buf, &mut dst).unwrap();
        assert_eq!(dst, i32::MIN);
        assert_eq!(size, 10);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0x07];
        let size = decode_int32(&buf, &mut dst).unwrap();
        assert_eq!(dst, i32::MAX);
        assert_eq!(size, 5);
    }

    /// Should decode an encoded numeric value into `int64` data type.
    #[test]
    fn decodes_int64() {
        let mut dst = 0;
        let buf = vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        let size = decode_int64(&buf, &mut dst).unwrap();
        assert_eq!(dst, i64::MIN);
        assert_eq!(size, 10);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let size = decode_int64(&buf, &mut dst).unwrap();
        assert_eq!(dst, i64::MAX);
        assert_eq!(size, 9);
    }

    /// Should decode an encoded numeric value into `uint32` data type.
    #[test]
    fn decodes_uint32() {
        let mut dst = 0;
        let buf = vec![0x00];
        let size = decode_uint32(&buf, &mut dst).unwrap();
        assert_eq!(dst, u32::MIN);
        assert_eq!(size, 1);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0x0f];
        let size = decode_uint32(&buf, &mut dst).unwrap();
        assert_eq!(dst, u32::MAX);
        assert_eq!(size, 5);
    }

    /// Should decode an encoded numeric value into `uint64` data type.
    #[test]
    fn decodes_uint64() {
        let mut dst = 0;
        let buf = vec![0x00];
        let size = decode_uint64(&buf, &mut dst).unwrap();
        assert_eq!(dst, u64::MIN);
        assert_eq!(size, 1);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        let size = decode_uint64(&buf, &mut dst).unwrap();
        assert_eq!(dst, u64::MAX);
        assert_eq!(size, 10);
    }
    
    /// Should decode an encoded numeric value into `sint32` data type.
    #[test]
    fn decodes_sint32() {
        let mut dst = 0;
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0x0f];
        let size = decode_sint32(&buf, &mut dst).unwrap();
        assert_eq!(dst, i32::MIN);
        assert_eq!(size, 5);
        let buf = vec![0xfe, 0xff, 0xff, 0xff, 0x0f];
        let size = decode_sint32(&buf, &mut dst).unwrap();
        assert_eq!(dst, i32::MAX);
        assert_eq!(size, 5);
    }

    /// Should decode an encoded numeric value into `sint64` data type.
    #[test]
    fn decodes_sint64() {
        let mut dst = 0;
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        let size = decode_sint64(&buf, &mut dst).unwrap();
        assert_eq!(dst, i64::MIN);
        assert_eq!(size, 10);
        let buf = vec![0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        let size = decode_sint64(&buf, &mut dst).unwrap();
        assert_eq!(dst, i64::MAX);
        assert_eq!(size, 10);
    }

    /// Should decode an encoded numeric value into `fixed32` data type.
    #[test]
    fn decodes_fixed32() {
        let mut dst = 0;
        let buf = vec![0x00];
        assert!(decode_fixed32(&buf, &mut dst).is_err());
        let buf = vec![0x00, 0x00, 0x00, 0x00];
        let size = decode_fixed32(&buf, &mut dst).unwrap();
        assert_eq!(dst, u32::MIN);
        assert_eq!(size, 4);
        let buf = vec![0xff, 0xff, 0xff, 0xff];
        let size = decode_fixed32(&buf, &mut dst).unwrap();
        assert_eq!(dst, u32::MAX);
        assert_eq!(size, 4);
    }

    /// Should decode an encoded numeric value into `fixed64` data type.
    #[test]
    fn decodes_fixed64() {
        let mut dst = 0;
        let buf = vec![0x00];
        assert!(decode_fixed64(&buf, &mut dst).is_err());
        let buf = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff];
        let size = decode_fixed64(&buf, &mut dst).unwrap();
        assert_eq!(dst, u64::MIN);
        assert_eq!(size, 8);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let size = decode_fixed64(&buf, &mut dst).unwrap();
        assert_eq!(dst, u64::MAX);
        assert_eq!(size, 8);
    }

    // /// Should decode an encoded numeric value into `sfixed32` data type.
    #[test]
    fn decodes_sfixed32() {
        let mut dst = 0;
        let buf = vec![0x00];
        assert!(decode_sfixed32(&buf, &mut dst).is_err());
        let buf = vec![0x00, 0x00, 0x00, 0x80];
        let size = decode_sfixed32(&buf, &mut dst).unwrap();
        assert_eq!(dst, i32::MIN);
        assert_eq!(size, 4);
        let buf = vec![0xff, 0xff, 0xff, 0x7f];
        let size = decode_sfixed32(&buf, &mut dst).unwrap();
        assert_eq!(dst, i32::MAX);
        assert_eq!(size, 4);
    }
    
    /// Should decode an encoded numeric value into `sfixed64` data type.
    #[test]
    fn decodes_sfixed64() {
        let mut dst = 0;
        let buf = vec![0x00];
        assert!(decode_sfixed64(&buf, &mut dst).is_err());
        let buf = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
        let size = decode_sfixed64(&buf, &mut dst).unwrap();
        assert_eq!(dst, i64::MIN);
        assert_eq!(size, 8);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f];
        let size = decode_sfixed64(&buf, &mut dst).unwrap();
        assert_eq!(dst, i64::MAX);
        assert_eq!(size, 8);
    }

    /// Should decode an encoded numeric value into `float` data type.
    #[test]
    fn decodes_float() {
        let mut dst = 0.0;
        let buf = vec![0x00];
        assert!(decode_float(&buf, &mut dst).is_err());
        let buf = vec![0xff, 0xff, 0x7f, 0xff];
        let size = decode_float(&buf, &mut dst).unwrap();
        assert_eq!(dst, f32::MIN);
        assert_eq!(size, 4);
        let buf = vec![0xff, 0xff, 0x7f, 0x7f];
        let size = decode_float(&buf, &mut dst).unwrap();
        assert_eq!(dst, f32::MAX);
        assert_eq!(size, 4);
    }

    /// Should decode an encoded numeric value into `double` data type.
    #[test]
    fn decodes_double() {
        let mut dst = 0.0;
        let buf = vec![0x00];
        assert!(decode_double(&buf, &mut dst).is_err());
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xef, 0xff];
        let size = decode_double(&buf, &mut dst).unwrap();
        assert_eq!(dst, f64::MIN);
        assert_eq!(size, 8);
        let buf = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xef, 0x7f];
        let size = decode_double(&buf, &mut dst).unwrap();
        assert_eq!(dst, f64::MAX);
        assert_eq!(size, 8);
    }
}
