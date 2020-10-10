// Encodes data bytes to Huffman's sequence.
pub fn encode(data: &[u8], codings: &Vec<(u8, u32)>) {

}

#[cfg(test)]
mod test {
    use super::*;

    fn codings() -> Vec<(u8, u32)> {
        vec![
            (2, 0),  // char(0) = 00   A
            (2, 1),  // char(1) = 01   B
            (3, 4),  // char(2) = 100  C
            (3, 5),  // char(3) = 101  D
            (2, 3),  // char(4) = 11   E
        ]
    }

    #[test]
    fn flattens_2bits() { 
        let speed = 2; // 2 bits at a time
        assert_eq!(encode(&[2, 1], &codings()), [
        //     (-1, [( 1, 0), ( 2, 0), ( 3, 0), ( 6, 0)]),
        //     ( 0, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 1, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     (-1, [( 4, 1), ( 4, 1), ( 5, 1), ( 5, 1)]),
        //     ( 2, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 3, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        //     ( 4, [(-1, 0), (-1, 0), (-1, 0), (-1, 0)]),
        ]);
    }
}
