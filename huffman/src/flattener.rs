use std::collections::HashMap;

/// Builds Huffman transition matrix for decoding Huffman sequence. The function
/// accepts the `speed` attribute which indicates the number of bits that the
/// decoder should read at the time. Note that increasing the speed (reading
/// multiple bites at a time), makes the decoder faster but also contributes to
/// a higher memory footprint.
/// 
/// 
/// 
pub fn flatten(codings: &[(u8, u32)], speed: u8) -> Vec<(i8, Vec<(isize, u8)>)> {
    let mut transitions: HashMap<usize, (i8, HashMap<String, (usize, usize)>)> = HashMap::new(); // H[<id>, (<ascii>, H[<key>, (<next-id>, <leftover-size>)])]
    transitions.insert(0, (-1, HashMap::new()));

    let mut paths: HashMap<String, usize> = HashMap::new();
    let mut last_id = 0;

    // map matrix transitions
    for (ascii, coding) in codings.iter().enumerate() {
        let mut key = String::new(); // start at root
        let mut id = 0; // root ID
        let sequence = coding_to_mbs(&coding); // MBS bite sequence
        
        for (i, chunk) in sequence.chunks(speed as usize).enumerate() {

            let leftovers = chunk.len() % speed as usize;
            let end = leftovers != 0 || sequence.len() == (i + 1) * speed as usize;

            let mut next_id = None;
            for k in build_key_candidates(&chunk, speed) {
                let apx = bits_to_string(&k); // current key partial

                let key = if leftovers != 0 { // last
                    let mut key = key.clone();
                    key.push_str(&apx);
                    key
                } else { // continue
                    key.push_str(&apx); 
                    key.clone()
                };

                let idx = *paths.entry(key.clone()).or_insert_with(|| {
                    let idx = if let Some(next_id) = next_id {
                        next_id
                    } else {
                        last_id += 1;
                        next_id = Some(last_id);
                        last_id
                    };
                    transitions.insert(idx, (-1, HashMap::new()));
                    idx
                });
                next_id = Some(idx);
                
                let transition = transitions.get_mut(&id).unwrap();
                transition.1.insert(apx, (idx, leftovers));
            }

            if let Some(next_id) = next_id {
                id = next_id; // continue at this level

                if end {
                    let transition = transitions.get_mut(&next_id).unwrap();
                    transition.0 = ascii as i8; // set ASCII
                }
            }
        }
    }
    
    // flatten matrix transitions
    let mut items = Vec::new();
    for id in 0..paths.len() - 1 {
        let transition = transitions.get(&id).unwrap();
        let ascii = transition.0;

        let mut pointers = (ascii, vec![(-1, 0); 2u32.pow(speed as u32) as usize]);
        for (key, (next_id, leftovers)) in transition.1.iter() {
            let key = usize::from_str_radix(key, 2).unwrap();
            pointers.1[key] = (*next_id as isize, *leftovers as u8);
        }
        items.push(pointers);
    }
    items
}

/// Converts the provided coding to the MSB format.
/// 
/// Codings that we get for each ASCII character by parsing the static Huffman
/// table are later stored to the ENCODE_TABLE in a minimal format. The
/// flattener needs to convert each coding back to the original Huffman MBS
/// binary format.
/// 
/// The hexadecimal number (second parameter) tells the binary value while the
/// provided length (first parameter) tells the length of the Huffman code so
/// the prepended zeros are not lost.
/// 
/// Example:
///     
///     From:        To:
///     (5, 0x2B) => 00101011
/// 
fn coding_to_mbs(coding: &(u8, u32)) -> Vec<bool> {
    format!("{:0>1$}", format!("{:b}", coding.1), coding.0 as usize)
        .chars()
        .map(|c| c == '1')
        .collect::<Vec<bool>>()
}

/// Returns the list of maching MBS keys for the provided bits. 
/// 
/// To construct a matrix for decoding Huffman sequence by readin N-bit we 
/// expect all chunks to be of size N. When the last chunk of a key is less then
/// N we need to fill the missing bits by generating all possible candidates.
/// 
/// Example:
///     
///     Sequence:    2-bit chunks:    Candidates: (1 leftover)
///     001001000 => 00 10 01 00 0 => 00 10 01 00 00
///                                   00 10 01 00 01
/// 
fn build_key_candidates(bits: &[bool], speed: u8) -> Vec<Vec<bool>> {
    let size = speed as usize - bits.len();
    if size == 0 {
        return vec![bits.to_vec()];
    }

    let mut combinations = vec![];
    for _ in 0..size {
        for i in 0..2 {
            combinations.push(
                bits.iter().cloned().chain(vec![i != 0]).collect()
            );
        }
    }
    combinations
}

/// Converts a list of bits into a string.
/// 
/// Example:
///     
///     Input:                  Output:
///     [true, false, false] => "100"
/// 
fn bits_to_string(bits: &[bool]) -> String {
    bits.iter()
        .map(|b| (*b as u8).to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    fn codings() -> Vec<(u8, u32)> {
        vec![
            (2, 0),  // char(0) = 00
            (2, 1),  // char(1) = 01
            (3, 4),  // char(2) = 100
            (3, 5),  // char(3) = 101
            (2, 3),  // char(4) = 11
        ]
    }

    #[test]
    fn flattens_2bits() { 
        let speed = 2; // 2 bits at a time
        // assert_eq!(flatten(&codings(), speed), [
        //     (-1, [( 1, 0), ( 2, 0), ( 3, 0), ( 6, 0)]),
        //     ( 0, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 1, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     (-1, [( 4, 1), ( 4, 1), ( 5, 1), ( 5, 1)]),
        //     ( 2, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 3, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 4, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        // ]);
    }
}
