use super::*;

/// Encode an integer number to the integer representation defined by HPACK
/// based on the pseudocode provided by the HPACK specification ([5.1]).
/// 
/// This is a generic function which is used in multiple places in the code
/// where a numeric value needs to be encoded into the HPACK format. The HPACK
/// integer representation allows for values of indefinite size. As specified by
/// the HPACK specification, an implementation has to set a limit for integer
/// values it accepts, as well as for the encoded length. This implementation
/// supports numbers up to the maximum value of the unsigned 32-bit integer.
/// 
/// We can specify `flags` which represent the leading bits of the first octet.
/// 
/// The `prefix` is a parameter of the integer representation. Its  size must be
/// between 1 and 8 bits. An integer starting at an octet boundary will have an
/// 8-bit prefix.
/// 
/// [5.1]: https://tools.ietf.org/html/rfc7541#section-5.1
pub fn encode_integer(value: u32, flags: u8, prefix_size: u8, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
    if prefix_size < 1 || prefix_size > 8 {
        return Err(EncoderError::InvalidPrefix);
    }

    let mask = ((1 << prefix_size) - 1) as u8; // max possible value of the first byte
    let flags = flags & 255 - mask as u8; // remove invalid flags

    if value < mask as u32 { // small enought to fit intothe first byte
        dst.push(flags | value as u8);
        return Ok(());
    }

    let mut value = value - mask as u32;
    dst.push(flags | mask as u8); // first byte
    while value >= 128 {
        dst.push(0b10000000 | value as u8); // byte with continuation flag
        value >>= 7;
    }
    dst.push(value as u8); // last byte
    Ok(())
}

/// Encodes a string to the string representation defined by HPACK ([5.2]).
/// 
/// When `huffman` is 'false' then the encoded data is the raw octets of the
/// string literal and when `huffman` is 'true', then the encoded data is the
/// Huffman encoding of the string literal.
/// 
/// [5.2]: https://tools.ietf.org/html/rfc7541#section-5.1
pub fn encode_string(data: Vec<u8>, huffman: bool, dst: &mut Vec<u8>) -> Result<(), EncoderError> {
    let (flags, bytes) = if huffman {
        let mut dst = Vec::new();
        if let Err(e) = httlib_huffman::encode(&data, &mut dst) {
            return Err(EncoderError::Huffman(e));
        }
        (0x80, dst) // set MSB to 1 indicating Huffman encoded literal
    } else {
        (0, data.to_vec()) // set MSB to 0 indicating plain text
    };

    let len = bytes.len();
    if len > u32::MAX as usize {
        return Err(EncoderError::IntegerOverflow);
    }

    encode_integer(len as u32, flags, 7, dst)?; // first byte
    dst.append(&mut bytes.to_vec()); // the rest of bytes

    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encodes_integer() {
        let examples = vec![
            (10,   0, 5, vec![10]), // https://tools.ietf.org/html/rfc7541#appendix-C.1.1
            (1337, 0, 5, vec![31, 154, 10]), // https://tools.ietf.org/html/rfc7541#appendix-C.1.2
            (42,   0, 8, vec![0b00101010]), // https://tools.ietf.org/html/rfc7541#appendix-C.1.3
            (127,  0, 7, vec![127, 0]),
            (255,  0, 8, vec![255, 0]),
            (254,  0, 8, vec![254]),
            (1,    0, 8, vec![1]),
            (0,    0, 8, vec![0]),
            (255,  0, 7, vec![127, 128, 1]),
            (10,   0b10000000, 5, vec![0b10001010]), // 3 MSB (flags) are 100
            (10,   0b01000000, 5, vec![0b01001010]), // 3 MSB (flags) are 010
            (10,   0b00010000, 5, vec![0b00001010]), // MSB > 3 should be ignored
            (1337, 0b01000000, 5, vec![0b01011111, 0b10011010, 0b00001010]), // custom flag 010XXXXX
        ];
        for (value, flags, prefix, res) in examples {
            let mut dst = Vec::new();
            encode_integer(value, flags, prefix, &mut dst).unwrap();
            assert_eq!(dst, res);
        }
    }

    #[test]
    fn encodes_string() {
        let examples = vec![
            (b"foo".to_vec(), false, vec![3, 102, 111, 111]), // plain test
            (b"foo".to_vec(), true,  vec![130, 148, 231]), // Huffman encoded
        ];
        for (value, huffman, bytes) in examples {
            let mut dst = Vec::new();
            encode_string(value, huffman, &mut dst).unwrap();
            assert_eq!(dst, bytes);
        }
    }
}
