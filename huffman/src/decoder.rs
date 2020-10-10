/// Decodes Huffman's sequence from the provided matrix. The matrix design
/// explains how many bits should be read at the time.
pub fn decode(sequence: &str, matrix: &Vec<Vec<(isize, i8, u8)>>) {

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
        // assert_eq!(decode("CB", speed), [
        // ]);
    }
}
