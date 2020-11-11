use super::*;

/// Decodes an integer number encoded with a given prefix size (in bits) based
/// on the pseudocode provided by the HPACK specification ([5.1.]).
/// 
/// The method assumes that the buffer `buf` contains the integer to be decoded,
/// with the first byte representing the octet that contains the prefix. The
/// result is written into `dst` and the number of bytes from the buffer that
/// were used is returned.
/// 
/// The HPACK integer representation allows for values of indefinite size. As
/// specified by the HPACK specification, an implementation has to set a limit
/// for integer values it accepts, as well as for the encoded length. This
/// implementation's maximum number of supported octets is set to `5` and is
/// chosen such that the maximum allowed `value` can never overflow an unsigned
/// 32-bit integer. The maximum value of any integer that can be encoded with
/// `5` octets is ~2^28. 
/// 
/// The `prefix` is a parameter of the integer representation. The prefix size
/// must be between 1 and 8 bits. An integer starting at an octet boundary will
/// have an 8-bit prefix.
/// 
/// **Integer value encoded within the 5-bit prefix ([5.1.], figure 2):**
/// 
/// ```txt
///   0   1   2   3   4   5   6   7
/// +---+---+---+---+---+---+---+---+
/// | ? | ? | ? |       value       |
/// +---+---+---+-------------------+
/// ```
/// 
/// **Integer value encoded after the 5-bit prefix ([5.1.], figure 3):**
/// 
/// ```txt
///   0   1   2   3   4   5   6   7
/// +---+---+---+---+---+---+---+---+
/// | ? | ? | ? | 1   1   1   1   1 |
/// +---+---+---+-------------------+
/// | 1 |    Value-(2^N-1) LSB      |
/// +---+---------------------------+
///                ...
/// +---+---------------------------+
/// | 0 |    Value-(2^N-1) MSB      |
/// +---+---------------------------+
/// ```
/// 
/// [5.1.]: https://tools.ietf.org/html/rfc7541#section-5.1
pub(crate) fn decode_integer(buf: &mut Vec<u8>, val: &mut u32, prefix_size: u8) -> Result<usize, DecoderError> {
    if prefix_size < 1 || prefix_size > 8 { // invalid prefix
        return Err(DecoderError::InvalidPrefix);
    }
    
    let mut total = 0; // once octet already read
    let byte = if buf.is_empty() {
        return Err(DecoderError::IntegerUnderflow);
    } else {
        total += 1;
        buf.remove(0)
    };

    let mask = ((1 << prefix_size) - 1) as u8; // max possible value of the first byte
    let mut value = (byte & mask) as u32;
    if value < (mask as u32) { // value fits in the prefix bits.
        *val = value;
        return Ok(total);
    }

    let mut shift = 0;
    loop {
        let byte = if buf.is_empty() {
            return Err(DecoderError::IntegerUnderflow);
        } else {
            total += 1;
            buf.remove(0)
        };
        
        value += ((byte & 0b01111111) as u32) << shift;
        shift += 7;

        if byte & 0b10000000 == 0 { // most significant bit is set (continuation)
            *val = value;
            return Ok(total);
        } else if total == 5 { // chosen limit of supported octet
            return Err(DecoderError::IntegerOverflow);
        }
    }
}

/// Decodes HPACK encoded string to plain test ([5.2.]).
/// 
/// The function uses the Huffman decoder which can decode a buffer with the
/// provided `speed`. More bits at a time mean faster decoding but at the same
/// time a higher memory footprint.
/// 
/// **String literal representation ([5.2.], figure 4):**
/// 
/// ```txt
///   0   1   2   3   4   5   6   7
/// +---+---+---+---+---+---+---+---+
/// | H |    String Length (7+)     |
/// +---+---------------------------+
/// |  String Data (Length octets)  |
/// +-------------------------------+
/// ```
/// 
/// [5.2.]: https://tools.ietf.org/html/rfc7541#section-5.2
pub(crate) fn decode_string(buf: &mut Vec<u8>, speed: DecoderSpeed, dst: &mut Vec<u8>) -> Result<usize, DecoderError> {
    let huffman = buf[0] & 128 == 128;

    let mut len = 0;
    decode_integer(buf, &mut len, 7)?;

    if len as usize > buf.len() {
        return Err(DecoderError::IntegerUnderflow);
    }

    let mut buf: Vec<u8> = buf.drain(0..len as usize).collect();
    if huffman { // Huffman encoded (MSB is set to 1)
        httlib_huffman::decode(&buf, dst, speed)?;
    } else { // Plain text (MSB is set to 0)
        dst.append(&mut buf);
    }

    Ok(1 + len as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Decodes a HPACK integer representation sequence into integer ([5.1.]).
    /// 
    /// [5.1.]: https://tools.ietf.org/html/rfc7541#section-5.1
    #[test]
    fn decodes_integer() {
        let examples = vec![
            (vec![10], 5, 10, 1), // https://tools.ietf.org/html/rfc7541#appendix-C.1.1
            (vec![31, 154, 10], 5, 1337, 3), // https://tools.ietf.org/html/rfc7541#appendix-C.1.2
            (vec![31 + 32, 154, 10], 5, 1337, 3), // with flags
            (vec![31 + 64, 154, 10], 5, 1337, 3), // with flags
            (vec![31, 154, 10, 111, 22], 5, 1337, 3), // with flags
            (vec![42], 8, 42, 1), // https://tools.ietf.org/html/rfc7541#appendix-C.1.3
            (vec![127, 0], 7, 127, 2),
            (vec![255, 0], 7, 127, 2),
            (vec![254], 8, 254, 1),
            (vec![1], 8, 1, 1),
            (vec![0], 8, 0, 1),
            (vec![127, 128, 1], 7, 255,  3),
            (vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF - 128], 8, 268435710, 5), // the largest allowed integer (2^28)
        ];
        for (mut value, prefix, res, size) in examples {
            let mut dst = 0;
            let total = decode_integer(&mut value, &mut dst, prefix).unwrap();
            assert_eq!(dst, res);
            assert_eq!(total, size);
        }
    }

    /// Decodes a HPACK string representation sequence into string ([5.2.]).
    /// 
    /// [5.2.]: https://tools.ietf.org/html/rfc7541#section-5.2
    #[test]
    fn decodes_string() {
        let examples = vec![
            (b"foo", vec![3, 102, 111, 111]), // plain test
            (b"foo", vec![130, 148, 231]), // Huffman encoded
        ];
        for (value, mut bytes) in examples {
            let mut dst = Vec::new();
            decode_string(&mut bytes, DecoderSpeed::FourBits, &mut dst).unwrap();
            assert_eq!(dst, value);
        }
    }
}
