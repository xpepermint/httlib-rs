
pub fn flatten(codings: &[(u8, u32)], speed: usize) -> Vec<Vec<(Option<usize>, Option<usize>, usize)>> { // next_id, ascii, leftover
    let blank_transition = generate_blank_transition(speed);

    let mut matrix: Vec<Vec<(Option<usize>, Option<usize>, usize)>> = Vec::new();
    matrix.push(blank_transition.clone());

    for (ascii, coding) in codings.iter().enumerate() {
        let leftover = (coding.0 as f32 / speed as f32).ceil() as usize * speed - coding.0 as usize;
        let mut id = 0; // current walk index in matrix
        
        for (path_index, keys) in generate_coding_paths(coding, speed).iter().enumerate() {

            if path_index == 0 { // create IDs for original path
                for key_index in 0..keys.len() - 1 { // the last key will be handled afterward
                    let key = keys[key_index];
                    let target = &matrix[id][key]; // should always exist
    
                    let next_id = if let Some(next_id) = target.0 {
                        next_id
                    } else {
                        matrix.push(blank_transition.clone());

                        let next_id = matrix.len() - 1;
                        let transitions = matrix.get_mut(id).unwrap();
                        let target = transitions.get_mut(key).unwrap();
                        target.0 = Some(next_id);

                        next_id
                    };
    
                    id = next_id;
                }
            } 
            
            let key = keys.last().unwrap(); // handle the last key of all path variants
            let transitions = matrix.get_mut(id).unwrap();
            let target = transitions.get_mut(*key).unwrap();
            target.1 = Some(ascii);
            target.2 = leftover;
        }
    }

    matrix
}

fn generate_blank_transition(speed: usize) -> Vec<(Option<usize>, Option<usize>, usize)> {
    let mut transition = Vec::new();

    for _ in 0..2u32.pow(speed as u32) {
        transition.push((None, None, 0));
    }

    transition
}

/// Za prejeti coding najprej splita bite v vektor ob bits velikosti glede na
/// podani `speed`. Ce ima zadnji bit leftover naredimo variante.
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

    #[cfg(feature = "decode1")]
    #[test]
    fn flattens_1bits() { 
        assert_eq!(
            flatten(&crate::encode::table::ENCODE_TABLE, 1),
            crate::decode::table1::DECODE_TABLE.to_vec()
        );
    }

    #[cfg(feature = "decode2")]
    #[test]
    fn flattens_2bits() { 
        assert_eq!(
            flatten(&crate::encode::table::ENCODE_TABLE, 2),
            crate::decode::table2::DECODE_TABLE.to_vec()
        );
    }

    #[cfg(feature = "decode3")]
    #[test]
    fn flattens_3bits() { 
        assert_eq!(
            flatten(&crate::encode::table::ENCODE_TABLE, 3),
            crate::decode::table3::DECODE_TABLE.to_vec()
        );
    }

    #[cfg(feature = "decode4")]
    #[test]
    fn flattens_4bits() { 
        assert_eq!(
            flatten(&crate::encode::table::ENCODE_TABLE, 4),
            crate::decode::table4::DECODE_TABLE.to_vec()
        );
    }

    #[cfg(feature = "decode5")]
    #[test]
    fn flattens_5bits() { 
        assert_eq!(
            flatten(&crate::encode::table::ENCODE_TABLE, 5),
            crate::decode::table5::DECODE_TABLE.to_vec()
        );
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
