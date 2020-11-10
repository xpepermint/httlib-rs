//! This module provides features for flattening Huffman tree and generating
//! translation matrix.
//! 
//! [HPACK] documentation provides an already prepared and for the web optimized
//! Huffman code for all [ASCII] characters. To implement the Huffman algorithm
//! for [HPACK] we have to first flatten this table to a two-dimensional matrix.
//! 
//! Let’s learn how to do this on a very simple example. Our algorithm will
//! enable the conversion of letters A, B, C, D, and E into a Huffman sequence.
//! The Huffman code for each letter is shown in the table below.
//! 
//! | Character | Huffman code
//! |-|-
//! | A | 00
//! | B | 01
//! | C | 100
//! | D | 101
//! | E | 11
//! 
//! We have decided to flatten the Huffman table into a matrix, enabling the
//! decoder to read Huffman bit-sequence 2-bits at a time. The illustration
//! below shows the table structure we need to fill in. 
//! 
//! | PATH | ID | SYM | LFT | 00 | 01 | 10 | 11
//! |-|-|-|-|-|-|-|-
//! | // | 0 | - | - | - | - | - | -
//! 
//! The first column PATH will serve for our notes in which we’ll store read
//! bits so we will know what sequence refers to what table row. Reading of each
//! character’s code always starts in the root row marked with `//`. The column
//! `ID` will store the unique name of the row. The first row is marked with
//! `0`. The column `SYM` will store characters (e.g. A). Field `LFT` will store
//! the information about the leftover bits. A leftover bit is a number of bits,
//! missing to reach the full bit chunk (in our case 2 bits). For example,
//! letter C and D have a leftover of 1, because to reach a round number of
//! bits, which is in our case 2 bits * N, 1 bit remains. Letters A, B, and E
//! have no leftover. The remaining columns represent the read chunk of 2 bits
//! for all its possible values ranging from `00` (0) to `11` (3). 
//! 
//! The table above will now be filled with data of sample Huffman coding. As
//! mentioned previously, we are reading the Hoffman code 2-bits at a time.
//! Let’s see how to insert data to the table for the first letter A.  
//! 
//! Letter A is represented with code `00`. Since there is no path `//00` for
//! this code in the first column, we create a new line with a new `ID`. There
//! is no leftover, and in the root line to column `00` we write the `ID` of the
//! newly established line. Since we read all the bits for the letter A, we also
//! write character A in the `SYM` column. 
//! 
//! | Path | ID | SYM | LFT | 00 | 01 | 10 | 11
//! |-|-|-|-|-|-|-|-
//! | // | 0 | - | - | 1 | - | - | -
//! | //00 | 1 | A | 0 | - | - | - | -
//! 
//! We then repeat this process for the letter B. The letter B is represented
//! with code `01`. Since there is no path `//01` for this code, we create a
//! new line with a new `ID`. There is no leftover, and in the root line in
//! column `01` we write the `ID` of the newly established line. Since we read
//! all the bits for the letter B, we also write character B to the `SYM`
//! column.
//! 
//! | Path | ID | SYM | LFT | 00 | 01 | 10 | 11
//! |-|-|-|-|-|-|-|-
//! | // | 0 | - | - | 1 | 2 | - | -
//! | //00 | 1 | A | 0 | - | - | - | -
//! | //01 | 2 | B | 0 | - | - | - | -
//! 
//! The process for the letter C is somewhat different since its number of bits
//! doesn’t correspond to 2-bits * N. The final bit is therefore missing, so we
//! claim that it has a leftover of 1. First, we read the first 2 bits and
//! insert them in the table following the same process as before. After that,
//! we read the remaining bit, while assuming that all the possible variations
//! of the missing bit exist. This is marked with `X`. Since one bit is missing,
//! we note this in the column `LFT`. 
//! 
//! | Path | ID | SYM | LFT | 00 | 01 | 10 | 11
//! |-|-|-|-|-|-|-|-
//! | // | 0 | - | - | 1 | 2 | 3 | -
//! | //00 | 1 | A | 0 | - | - | - | -
//! | //01 | 2 | B | 0 | - | - | - | -
//! | //10 | 2 | B | 0 | - | - | - | -
//! | //10 | 3 | - | - | 4 | 4 | - | -
//! | //100X | 4 | C | 1 | - | - | - | -
//! 
//! We repeat the process for letters D and E. The final table should look like
//! this: 
//! 
//! | Path | ID | SYM | LFT | 00 | 01 | 10 | 11
//! |-|-|-|-|-|-|-|-
//! | // | 0 | - | - | 1 | 2 | 3 | 6
//! | //00 | 1 | A | 0 | - | - | - | -
//! | //01 | 2 | B | 0 | - | - | - | -
//! | //10 | 2 | B | 0 | - | - | - | -
//! | //10 | 3 | - | - | 4 | 4 | 5 | 5
//! | //100X | 4 | C | 1 | - | - | - | -
//! | //101X | 5 | D | 1 | - | - | - | -
//! | //11 | 6 | E | 0 | - | - | - | -
//! 
//! Note that it would be correct to replace the variants marked with X with
//! actual possible paths.
//! 
//! | Path | ID | SYM | LFT | 00 | 01 | 10 | 11
//! |-|-|-|-|-|-|-|-
//! | // | 0 | - | - | 1 | 2 | 3 | 6
//! | //00 | 1 | A | 0 | - | - | - | -
//! | //01 | 2 | B | 0 | - | - | - | -
//! | //10 | 2 | B | 0 | - | - | - | -
//! | //10 | 3 | - | - | 4 | 4 | 5 | 5
//! | //1000 | 4 | C | 1 | - | - | - | -
//! | //1001 | 4 | C | 1 | - | - | - | -
//! | //1010 | 5 | D | 1 | - | - | - | -
//! | //1011 | 5 | D | 1 | - | - | - | -
//! | //11 | 6 | E | 0 | - | - | - | -
//! 
//! The flattened form of the Huffman tree] in the form of a matrix plays a
//! crucial role in the process of decoding. We now have an idea of what the
//! process of decoding looks like, using this matrix. This module uses this
//! exact technic for creating N-bits translation matrix. 
//! 
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [ASCII]: https://en.wikipedia.org/wiki/ASCII

use crate::DecoderSpeed;

/// Generates a translation matrix that can be used to decode an encoded
/// content. The function expects the `speed` attribute which represents the
/// number of bits that the decoder will read at a time when processing bytes.
/// The speed attribute can be between 1 and 5 bits. The higher number will
/// have a positive effect on performance but possibly a higher memory usage.
/// 
/// **Example:**
/// 
/// ```rs
/// use httlib_huffman::encoder::table::ENCODE_TABLE;
/// 
/// let speed = DecoderSpeed::FourBits; // decoder will read 4 bits at a time
/// let table = flatten(&ENCODE_TABLE, speed);
/// ```
pub fn flatten(
    codings: &[(u8, u32)],
    speed: DecoderSpeed,
) -> Vec<Vec<(Option<u8>, Option<u16>, u8)>> { // next_id, ascii, leftover
    let speed = speed as usize;
    let blank_transition = generate_blank_transition(speed);

    let mut table: Vec<Vec<(Option<u8>, Option<u16>, u8)>> = Vec::new();
    table.push(blank_transition.clone());

    for (ascii, coding) in codings.iter().enumerate() {
        let leftover = (coding.0 as f32 / speed as f32).ceil() as usize * speed - coding.0 as usize;
        let mut id = 0; // current walk index in table
        
        for (path_index, keys) in generate_coding_paths(coding, speed).iter().enumerate() {

            if path_index == 0 { // create IDs for original path
                for key_index in 0..keys.len() - 1 { // the last key will be handled afterward
                    let key = keys[key_index];
                    let target = &table[id][key]; // should always exist
    
                    let next_id = if let Some(next_id) = target.0 {
                        next_id
                    } else {
                        table.push(blank_transition.clone());

                        let next_id = (table.len() - 1) as u8;
                        let transitions = table.get_mut(id).unwrap();
                        let target = transitions.get_mut(key).unwrap();
                        target.0 = Some(next_id);

                        next_id
                    };
    
                    id = next_id as usize;
                }
            } 
            
            let key = keys.last().unwrap(); // handle the last key of all path variants
            let transitions = table.get_mut(id).unwrap();
            let target = transitions.get_mut(*key).unwrap();
            target.1 = Some(ascii as u16);
            target.2 = leftover as u8;
        }
    }

    table
}

/// Generates a black transition object based on the provided speed attribute.
fn generate_blank_transition(speed: usize) -> Vec<(Option<u8>, Option<u16>, u8)> {
    let mut transition = Vec::new();

    for _ in 0..2u32.pow(speed as u32) {
        transition.push((None, None, 0));
    }

    transition
}

/// Generates all key paths for the particular coding. If the key has leftovers
/// the function will fill that gap with all possible variants.
/// 
/// ```tst
/// Code:       [1100, 0000, 1110, 011X]
/// Variants:   [1100, 0000, 1110, 0110]
///             [1100, 0000, 1110, 0111]
/// ```
fn generate_coding_paths(coding: &(u8, u32), speed: usize) -> Vec<Vec<usize>> {
    let mut bits: u32 = 0; // HPACK value can be up to 32 bits
    let chunks_len = (coding.0 as f32 / speed as f32).ceil() as usize;
    let chunk_max = 2u32.pow(speed as u32) as usize - 1;
    let leftover = chunks_len * speed - coding.0 as usize;
    bits |= coding.1;
    bits <<= leftover;

    let mut chunks = vec![];
    for i in 0..chunks_len {
        let mut chunk = (bits >> (chunks_len - i - 1) * speed) as usize;
        chunk &= chunk_max;
        chunks.push(chunk as usize);
    }
    
    let mut variants: Vec<Vec<usize>> = vec![];
    let chunk_last = chunks.pop().unwrap();
    let variants_len = 2u32.pow(leftover as u32) as usize;
    for i in 0..variants_len {
        let mut chunks = chunks.clone();
        chunks.push(chunk_last + i);
        variants.push(chunks);
    }

    variants
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_encoding_table() -> &'static [(u8, u32)] {
        &[
            (13, 0x1ff8),
            (23, 0x7fffd8),
            (28, 0xfffffe2),
            (28, 0xfffffe3),
            (28, 0xfffffe4),
            (28, 0xfffffe5),
            (28, 0xfffffe6),
            (28, 0xfffffe7),
            (28, 0xfffffe8),
        ]
    }
    
    /// Should generate a translation matrix that allows for decoding Huffman
    /// sequence by reading 1 bit at a time.
    #[test]
    fn flattens_1bits() { 
        let table = flatten(&sample_encoding_table(), DecoderSpeed::OneBit);
        assert_eq!(table.len(), 41);

        let target = &table[2][1];
        assert_eq!(target.0, Some(3));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    /// Should generate a translation matrix that allows for decoding Huffman
    /// sequence by reading 2 bits at a time.
    #[test]
    fn flattens_2bits() { 
        let table = flatten(&sample_encoding_table(), DecoderSpeed::TwoBits);
        assert_eq!(table.len(), 20);

        let target = &table[1][3];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    /// Should generate a translation matrix that allows for decoding Huffman
    /// sequence by reading 3 bits at a time.
    #[test]
    fn flattens_3bits() { 
        let table = flatten(&sample_encoding_table(), DecoderSpeed::ThreeBits);
        assert_eq!(table.len(), 16);

        let target = &table[1][7];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    /// Should generate a translation matrix that allows for decoding Huffman
    /// sequence by reading 4 bits at a time.
    #[test]
    fn flattens_4bits() { 
        let table = flatten(&sample_encoding_table(), DecoderSpeed::FourBits);
        assert_eq!(table.len(), 9);

        let target = &table[1][15];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    /// Should generate a translation matrix that allows for decoding Huffman
    /// sequence by reading 5 bits at a time.
    #[test]
    fn flattens_5bits() { 
        let table = flatten(&sample_encoding_table(), DecoderSpeed::FiveBits);
        assert_eq!(table.len(), 8);

        let target = &table[1][31];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    /// Should generate all key paths variants for the codings with leftover.
    #[test]
    fn generates_coding_paths() {
        assert_eq!(generate_coding_paths(&(14, 12345), 4), vec![ // code=11000000|111001XX, len=14
            vec![12, 0, 14, 4], // [1100, 0000, 1110, 0100]
            vec![12, 0, 14, 5], // [1100, 0000, 1110, 0101]
            vec![12, 0, 14, 6], // [1100, 0000, 1110, 0110]
            vec![12, 0, 14, 7], // [1100, 0000, 1110, 0111]
        ]);
        assert_eq!(generate_coding_paths(&(13, 2616), 4), vec![ // code=01010001|11000XXX, let=13
            vec![5, 1, 12, 0], // [0101, 0000, 1110, 0000]
            vec![5, 1, 12, 1], // [0101, 0000, 1110, 0001]
            vec![5, 1, 12, 2], // [0101, 0000, 1110, 0010]
            vec![5, 1, 12, 3], // [0101, 0000, 1110, 0011]
            vec![5, 1, 12, 4], // [0101, 0000, 1110, 0100]
            vec![5, 1, 12, 5], // [0101, 0000, 1110, 0101]
            vec![5, 1, 12, 6], // [0101, 0000, 1110, 0110]
            vec![5, 1, 12, 7], // [0101, 0000, 1110, 0111]
        ]);
    }
}
