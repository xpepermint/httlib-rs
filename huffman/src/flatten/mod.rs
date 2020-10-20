//! This module provides features for flattening [Huffman tree] and generating
//! translation matrixs.
//! 
//! [Huffman tree]: https://en.wikipedia.org/wiki/Huffman_coding

/// Generates a translation matrix that can be used to decode a decoded content.
/// The function expects the `speed` attribute which represents the number of
/// bits that the decoder will read at a time when decoding an encoded sequence. 
/// The speed attribute can be between 1 bit and 5 bits. The higher number will
/// have a positive effect on performance but a higher more footprint.
/// 
/// ```rs
/// use httlib_huffman::encode::table::ENCODE_TABLE;
/// 
/// let speed = 4; // decoder will read 4 bits at a time
/// let table = flatten(&ENCODE_TABLE, speed);
/// ```
pub fn flatten(codings: &[(u8, u32)], speed: usize) -> Vec<Vec<(Option<u8>, Option<u16>, u8)>> { // next_id, ascii, leftover
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
    
    #[test]
    fn flattens_1bits() { 
        let table = flatten(&sample_encoding_table(), 1);
        assert_eq!(table.len(), 41);

        let target = &table[2][1];
        assert_eq!(target.0, Some(3));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    #[test]
    fn flattens_2bits() { 
        let table = flatten(&sample_encoding_table(), 2);
        assert_eq!(table.len(), 20);

        let target = &table[1][3];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    #[test]
    fn flattens_3bits() { 
        let table = flatten(&sample_encoding_table(), 3);
        assert_eq!(table.len(), 16);

        let target = &table[1][7];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    #[test]
    fn flattens_4bits() { 
        let table = flatten(&sample_encoding_table(), 4);
        assert_eq!(table.len(), 9);

        let target = &table[1][15];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

    #[test]
    fn flattens_5bits() { 
        let table = flatten(&sample_encoding_table(), 5);
        assert_eq!(table.len(), 8);

        let target = &table[1][31];
        assert_eq!(target.0, Some(2));
        assert_eq!(target.1, None);
        assert_eq!(target.2, 0);
    }

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
