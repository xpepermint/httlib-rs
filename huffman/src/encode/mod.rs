mod error;

pub use error::*;

/// Encodes the provided `src` bytes and populates the `dst` with the sequance
/// of Huffman codes.
/// 
/// Example:
/// 
/// ```rs
/// use httlib_huffman::encode;
/// 
/// match encode(b"Hello world!", &mut dst) {
///     Ok(sequence) => { ... },
///     Err(err) => { ... },
/// }
/// ```
pub fn encode(src: &[u8], dst: &mut Vec<u8>) -> Result<(), EncodeError> {
    let mut bits: u64 = 0;
    let mut bits_left = 40;
    let codings = crate::data::encoder::ENCODE_TABLE; // parsed huffman table

    for &byte in src {
        let (code_len, code) = match codings.get(byte as usize) {
            Some(coding) => coding,
            None => return Err(EncodeError::InvalidAscii),
        };

        bits |= (*code as u64) << (bits_left - code_len); // shift and add old and new numbers
        bits_left -= code_len;

        while bits_left <= 32 {
            dst.push((bits >> 32) as u8);

            bits <<= 8; // add more room for the next character
            bits_left += 8;
        }
    }

    if bits_left != 40 { // finalize with EOS
        bits |= (1 << bits_left) - 1; // add EOS and pedding
        dst.push((bits >> 32) as u8);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encodes_bytes() { 
        let mut dst = Vec::new();
        encode(b" ", &mut dst).unwrap();
        encode(b"!", &mut dst).unwrap();
        encode(b"\"", &mut dst).unwrap();
        encode(b"#", &mut dst).unwrap();
        encode(b"$", &mut dst).unwrap();
        encode(b"%", &mut dst).unwrap();
        encode(b"&", &mut dst).unwrap();
        encode(b"'", &mut dst).unwrap();
        encode(b"(", &mut dst).unwrap();
        encode(b")", &mut dst).unwrap();
        encode(b"*", &mut dst).unwrap();
        encode(b"+", &mut dst).unwrap();
        encode(b",", &mut dst).unwrap();
        encode(b"-", &mut dst).unwrap();
        encode(b".", &mut dst).unwrap();
        encode(b"/", &mut dst).unwrap();
        encode(b"0", &mut dst).unwrap();
        encode(b"1", &mut dst).unwrap();
        encode(b"2", &mut dst).unwrap();
        encode(b"3", &mut dst).unwrap();
        encode(b"4", &mut dst).unwrap();
        encode(b"5", &mut dst).unwrap();
        encode(b"6", &mut dst).unwrap();
        encode(b"7", &mut dst).unwrap();
        encode(b"8", &mut dst).unwrap();
        encode(b"9", &mut dst).unwrap();
        encode(b":", &mut dst).unwrap();
        encode(b";", &mut dst).unwrap();
        encode(b"<", &mut dst).unwrap();
        encode(b"=", &mut dst).unwrap();
        encode(b">", &mut dst).unwrap();
        encode(b"?", &mut dst).unwrap();
        encode(b"@", &mut dst).unwrap();
        encode(b"A", &mut dst).unwrap();
        encode(b"B", &mut dst).unwrap();
        encode(b"C", &mut dst).unwrap();
        encode(b"D", &mut dst).unwrap();
        encode(b"E", &mut dst).unwrap();
        encode(b"F", &mut dst).unwrap();
        encode(b"G", &mut dst).unwrap();
        encode(b"H", &mut dst).unwrap();
        encode(b"I", &mut dst).unwrap();
        encode(b"J", &mut dst).unwrap();
        encode(b"K", &mut dst).unwrap();
        encode(b"L", &mut dst).unwrap();
        encode(b"M", &mut dst).unwrap();
        encode(b"N", &mut dst).unwrap();
        encode(b"O", &mut dst).unwrap();
        encode(b"P", &mut dst).unwrap();
        encode(b"Q", &mut dst).unwrap();
        encode(b"R", &mut dst).unwrap();
        encode(b"S", &mut dst).unwrap();
        encode(b"T", &mut dst).unwrap();
        encode(b"U", &mut dst).unwrap();
        encode(b"V", &mut dst).unwrap();
        encode(b"W", &mut dst).unwrap();
        encode(b"X", &mut dst).unwrap();
        encode(b"Y", &mut dst).unwrap();
        encode(b"Z", &mut dst).unwrap();
        encode(b"[", &mut dst).unwrap();
        encode(b"\\", &mut dst).unwrap();
        encode(b"]", &mut dst).unwrap();
        encode(b"^", &mut dst).unwrap();
        encode(b"_", &mut dst).unwrap();
        encode(b"`", &mut dst).unwrap();
        encode(b"a", &mut dst).unwrap();
        encode(b"b", &mut dst).unwrap();
        encode(b"c", &mut dst).unwrap();
        encode(b"d", &mut dst).unwrap();
        encode(b"e", &mut dst).unwrap();
        encode(b"f", &mut dst).unwrap();
        encode(b"g", &mut dst).unwrap();
        encode(b"h", &mut dst).unwrap();
        encode(b"i", &mut dst).unwrap();
        encode(b"j", &mut dst).unwrap();
        encode(b"k", &mut dst).unwrap();
        encode(b"l", &mut dst).unwrap();
        encode(b"m", &mut dst).unwrap();
        encode(b"n", &mut dst).unwrap();
        encode(b"o", &mut dst).unwrap();
        encode(b"p", &mut dst).unwrap();
        encode(b"q", &mut dst).unwrap();
        encode(b"r", &mut dst).unwrap();
        encode(b"s", &mut dst).unwrap();
        encode(b"t", &mut dst).unwrap();
        encode(b"u", &mut dst).unwrap();
        encode(b"v", &mut dst).unwrap();
        encode(b"w", &mut dst).unwrap();
        encode(b"x", &mut dst).unwrap();
        encode(b"y", &mut dst).unwrap();
        encode(b"z", &mut dst).unwrap();
        encode(b"{", &mut dst).unwrap();
        encode(b"|", &mut dst).unwrap();
        encode(b"}", &mut dst).unwrap();
        encode(b"~", &mut dst).unwrap();
        assert_eq!(dst, vec![
            83, 254, 63, 254, 127, 255, 175, 255, 207, 87, 248, 255, 95, 254,
            191, 254, 255, 249, 255, 127, 250, 91, 95, 99, 7, 15, 23, 103, 107,
            111, 115, 119, 123, 127, 185, 251, 255, 249, 131, 255, 191, 255, 63,
            255, 215, 135, 187, 189, 191, 193, 195, 197, 199, 201, 203, 205,
            207, 209, 211, 213, 215, 217, 219, 221, 223, 225, 227, 229, 252,
            231, 253, 255, 223, 255, 254, 31, 255, 231, 255, 243, 139, 255, 251,
            31, 143, 39, 147, 47, 151, 155, 159, 55, 233, 235, 163, 167, 171,
            63, 175, 237, 179, 71, 79, 183, 239, 241, 243, 245, 247, 255, 253,
            255, 159, 255, 247, 255, 239,
        ]);
        dst.clear();
        encode(b"Hello world!", &mut dst).unwrap();
        assert_eq!(dst, &[198, 90, 40, 58, 158, 15, 101, 18, 127, 31]);
    }
}
