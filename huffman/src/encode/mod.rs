//! An implementation of the [Canonical Huffman] code for [HPACK].
//! 
//! Header Compression format for [HTTP/2], known as [HPACK], foresees the use
//! of the Huffman algorithm for encoding header literal values. This
//! contributes to the additional decrease in the quantity of data, transferred
//! with each web request and response.
//! 
//! A Huffman code is a particular type of optimal prefix code that is commonly
//! used for lossless data compression. The process of finding or using such a
//! code proceeds by means of Huffman coding, an algorithm developed by David A.
//! Huffman. The output from Huffman's algorithm can be viewed as a
//! variable-length code table for encoding a source symbol (such as a character
//! in a file). The algorithm derives this table from the estimated probability
//! or frequency of occurrence (weight) for each possible value of the source
//! symbol. As in other entropy encoding methods, more common symbols are
//! generally represented using fewer bits than less common symbols. Huffman's
//! method can be efficiently implemented, finding a code in time linear to the
//! number of input weights if these weights are sorted. (Source: Wikipedia)
//! [HPACK] compression entails a pre-created [Canonical Huffman] code table
//! for encoding ASCII characters to the Huffman sequence. A [Canonical Huffman]
//! code is a particular type of Huffman code with unique properties that allow
//! it to be described in a very compact manner. The advantage of a
//! [Canonical Huffman] tree is that one can encode data in fewer bits than with
//! a fully described tree. In the aforementioned table are the Huffman codes
//! for each ASCII character with a length up to 32 bits (4x by 8 fields with
//! value 0 or 1), in the form of base-2 integer, aligned on the most
//! significant bit (MSB is the bit farthest to the left).
//! 
//! Encoding is relatively easy since we are replacing the individual characters
//! with the Huffman code. We add an EOS sign at the end and the desired
//! padding, which should be up to 7 bits long.
//! 
//! The Huffman encoder implementation illustration:
//! 
//! ```txt
//! [add "!"]        1111111000
//! [add "$"]        11111110001111111111001
//! [add "%"]        11111110001111111111001010101 (fix length)
//! [add "&"]        1111111000111111111100101010111111000
//! [add "A"]        1111111000111111111100101010111111000100001
//! [add EOS]        1111111000111111111100101010111111000100001111111111111111111111111111111
//! [add padding]    11111110001111111111001010101111110001000011111111111111111111111111111111111111
//! 
//! [result]         [254   ][63    ][242   ][175   ][196   ][63    ]
//!                  111111100011111111110010101011111100010000111111
//! ```
//! 
//! The illustration shows how the encoder iterates through all the ASCII
//! characters and replaces them with the Huffman code. Each line ends with the
//! EOS character and 7-bit padding is added.
//! 
//! While adding the Hoffman code to the sequence, the length of the added code
//! must exactly match the number of bits specified in the documentation.
//! Working with Huffman codes in bytes and then converting them to other types,
//! such as strings, could remove the prepended zeros. In such cases, we have to
//! do some plumbing to ensure all bits are there (an example of this would be
//! the character "%").
//! Implementation could be achieved by manipulating a string of ones and zeros.
//! However, for more complex systems such as high-performance web servers, this
//! would not be sustainable from the performance perspective. To manage
//! resources accordingly, we require innovation so the investments are
//! protected.
//! 
//! A replacement of the string with characters such as numbers, which are more
//! appropriate for computers, and the use of bitwise operators gives a
//! significant increase in performance. Before this can be done, we need to
//! have an understanding of how the numbers are added. Although we are all
//! aware of what "1+2=3" is, or what is a concatenation of a string such as
//! "aa+bb=aabb", in bit operations, these rules are not quite so obvious. Let's
//! see an example of the addition with bits directly:
//! 
//! ```txt
//!        1 +        2 =        3
//! 00000001 | 00000010 = 00000011
//! ```
//! 
//! For the sum of two bit numbers, we used the bitwise operator `|` which
//! serves as a sign for addition "+" in our example. Its rule is to trace the
//! bits of both numbers and, if a 0 or a 1 is found on the same spot, change
//! their value to 1, while setting the value to 0 in other cases. This
//! understanding now enables us to re-implement the example above.
//! 
//! Instead of a string, we will use a u64 data type storing a string of 64
//! bits. We could also use a data type with a larger capacity (such as u128),
//! but u64 is sufficient. The storage requirement is 32 bits, which is the
//! maximum length of the individual Huffman code plus an extra byte (8) for the
//! surplus cell, meaning that we need 40 bits of storage altogether.
//! 
//! The illustration below shows individual steps for encoding a string of
//! characters as in the example above, while the encoding is carried out with
//! the use of numbers and bitwise operators.
//! 
//! ```txt
//! [add "!"]        111111100000000000000000000000000000000000000000
//! [add "$"]        11111110001111111111001000000000000000000000000000000000
//! [add "%"]        1111111000111111111100101010100000000000000000000000000000000000 (fix length)
//! [add "&"]                  11111111110010101011111100000000000000000000000000000000000000
//! [add "A"]                        1111001010101111110001000010000000000000000000000000000000000000
//! [add EOS]                        1111001010101111110001000011111111111111111111111111111110000000
//! [add padding]                    1111001010101111110001000011111111111111111111111111111111111111
//! 
//! [result]         [254   ][63    ][242   ][175   ][196   ][63    ]
//!                  111111100011111111110010101011111100010000111111
//! ```
//! 
//! Although the illustration is quite similar to the previous one, it is much
//! more colorful. It is also apparent that a string of bits is getting shorter
//! on the left and longer on the right end.
//! When the Huffman code is added to the string for the individual character,
//! the algorithm immediately ensures 32 free bit spaces where the next
//! character will be added. This is achieved by the so-called shifting bits
//! using the "<<" bitwise operator. Since we are dealing with bit numbers, we
//! always rotate for 1 or more bytes, dependent on the required capacity,
//! meaning for 8*N bits. It might not be obvious but it is interesting that,
//! by rotating bits and adding the new Huffman character, we are adding numbers
//! in the same way as we did in the simple example previously presented.
//! 
//! If looked at separately, the Huffman algorithm is quite simple. But when we
//! don't intend to only implement it, but we are, instead, interested in the
//! maximization of the performance and lowering of used resources, things get
//! more complicated. The performance and quality of our solution are,
//! therefore, comparable to the implementation in some well-known web servers.
//! 
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [HTTP/2]: https://tools.ietf.org/html/rfc7540
//! [Canonical Huffman]: https://en.wikipedia.org/wiki/Canonical_Huffman_code
mod error;
pub mod table;

pub use error::*;

/// Encodes the provided `src` bytes and populates the `dst` with the sequance
/// of Huffman codes.
/// 
/// ```rs
/// use httlib_huffman::encode;
/// 
/// let mut dst = Vec::new();
/// match encode(b"Hello world!", &mut dst) {
///     Ok(sequence) => { ... },
///     Err(err) => { ... },
/// }
/// ```
pub fn encode(src: &[u8], dst: &mut Vec<u8>) -> Result<(), EncodeError> {
    let mut bits: u64 = 0;
    let mut bits_left = 40;
    let codings = self::table::ENCODE_TABLE; // parsed huffman table

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
        assert_eq!(dst, &[
            198, 90, 40, 58, 158, 15, 101, 18, 127, 31,
        ]);
    }
}
