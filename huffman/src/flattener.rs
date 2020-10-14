use std::collections::HashMap;

pub fn flatten2(codings: &[(u8, u32)], speed: usize) -> Vec<Vec<(Option<usize>, Option<usize>, usize)>> { // next_id, ascii, leftover
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

    for (id, row) in matrix.iter().enumerate() {
        println!("id: {:?}", id);
        for col in row.iter() {
            println!("    {:?}", col);
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

/// Builds Huffman transition matrix for decoding Huffman sequence. The function
/// accepts the `speed` attribute which indicates the number of bits that the
/// decoder should read at the time. Note that increasing the speed (reading
/// multiple bites at a time), makes the decoder faster but also contributes to
/// a higher memory footprint.
/// 
/// 
/// 
/// 
/// 
pub fn flatten(codings: &[(u8, u32)], speed: u8) -> Vec<(isize, usize, Vec<isize>)> { // next_id, 
    let mut transitions: HashMap<usize, (isize, usize, HashMap<String, usize>)> = HashMap::new(); // H[<id>, (<ascii>, <leftovers>, H[<key>, <next-id>])]
    transitions.insert(0, (-1, 0, HashMap::new()));

    let mut paths: HashMap<String, usize> = HashMap::new(); // H{<key>, <id>}
    let mut last_id = 0;

    // map matrix transitions
    for (ascii, coding) in codings.iter().enumerate() {
        let mut key = String::new(); // start at root
        let mut id = 0; // root ID
        let sequence = coding_to_mbs(&coding); // MBS bite sequence

        let chunks = sequence.chunks(speed as usize).collect::<Vec<&[bool]>>();

        for (i, chunk) in chunks.iter().enumerate() {
            let leftovers = speed as usize - chunk.len();
            let end = leftovers != 0 || sequence.len() == (i + 1) * speed as usize;
            
            let mut next_id = None;
            for k in build_key_candidates(&chunk, speed) {
                let apx = bits_to_string(&k); // current key partial

                // if ascii == 2 {
                    println!("candidate: --------------");
                    println!("     chunk: {:?} | {:?} ", chunk, k);
                    println!("        id:        {}", id);
                // }

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
                        transitions.insert(last_id, (-1, 0, HashMap::new()));
                        last_id
                    };
                    idx
                });
                next_id = Some(idx);
                
                let transition = transitions.get_mut(&id).unwrap();
                transition.2.insert(apx.clone(), idx);

                if end {
                    let transition = transitions.get_mut(&idx).unwrap();
                    transition.0 = ascii as isize; // set ASCII
                    transition.1 = leftovers;
                    // if ascii == 2 {
                        println!("  ascii:     {}", ascii);
                        println!("  leftovers: {}", leftovers);
                    // }
                }
            }
            id = next_id.unwrap(); // continue at this level
        }

        // if id >= 52 {
        //     break;
        // }
    }

    // for id in 0..transitions.len() {
    //     let transition = transitions.get(&id);
    //     println!("transition: {} | {:?}", id, transition);
    // }


    let mut items = Vec::new();
    for id in 0..(last_id + 1) {
        let transition = transitions.get(&id).unwrap();
        let ascii = transition.0;
        let leftovers = transition.1;

        let mut pointers = (ascii, leftovers, vec![-1 as isize; 2u32.pow(speed as u32) as usize]);
        for (key, next_id) in transition.2.iter() {
            let key = usize::from_str_radix(key, 2).unwrap();
            pointers.2[key] = *next_id as isize;
        }
        items.push(pointers);
    }



    // for (id, item) in items.iter().enumerate() {
    //     println!("item: {:?} | {:?}", id, item);
    // }

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
    // TODO: Tole je error!!!!! Odstej koliko res moras podaljsat string.
    println!("LEN: {:?}, BITS: {:b}", coding.0, coding.1);
    println!("     {:?}", format!("{:0>1$}", format!("{:b}", coding.1), coding.0 as usize));
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
    let leftovers = speed as usize - bits.len();
    let variants = 2u32.pow(leftovers as u32);
    if leftovers == 0 {
        return vec![bits.to_vec()];
    }

    let mut combinations = vec![];
    for i in 0..variants {
        let mut tail = format!("{:0>1$}", format!("{:b}", i), leftovers as usize)
            .chars()
            .map(|v| v != '0')
            .collect::<Vec<bool>>();

        let mut candidate = bits.clone().to_vec();
        candidate.append(&mut tail);
        combinations.push(candidate);
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

    #[test]
    fn flattens_2bits() { 
        // let speed = 4; // 2 bits at a time

        flatten2(&crate::data::encoder::ENCODE_TABLE, 4);
        // flatten(&crate::data::encoder::ENCODE_TABLE, 4);
        // assert_eq!(flatten(&codings(), 2), [
        //     (-1, 0, vec![( 1, 0), ( 2, 0), ( 3, 0), ( 6, 0)]),
        //     ( 0, 0, vec![(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 1, 0, vec![(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     (-1, 0, vec![( 4, 1), ( 4, 1), ( 5, 1), ( 5, 1)]),
        //     ( 2, 0, vec![(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 3, 0, vec![(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 4, 0, vec![(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        // ]);
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
