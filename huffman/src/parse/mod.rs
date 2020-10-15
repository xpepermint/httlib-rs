//! [HPACK] compression provides a pre-created [Huffman code] table for encoding
//! [ASCII] characters to the Huffman sequence. This Huffman code was generated
//! from statistics obtained on a large sample of HTTP headers.
//! 
//! The parser module is responsible for parsing the [Huffman code] table into
//! the static Rust source code. This module was used to create the ENCODE_TABLE
//! constant which can be found in the crate::encode::table module.
//! 
//! You will probably never use this module while developing applications thus
//! you have to enable by specifying the "parse" feature.
//! 
//! [ASCII]: https://en.wikipedia.org/wiki/ASCII
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [Huffman code]: https://tools.ietf.org/html/rfc7541#appendix-B

/// Parses the HPACK's static Huffman table. The function expects data to be in
/// format as provided by the spec (7.2).
pub fn parse(data: &str) -> Vec<(u16, u32)> {
    let lines = data.lines();
    let mut codings = vec![];

    for line in lines {
        let coding = parse_line(line);
        codings.push(coding);
    }

    codings
}

/// Parses a single line of the static Huffman table. The output returned
/// contains a tuple of the number of bits for the code representing the symbol
/// and Huffman LSB value.
fn parse_line(line: &str) -> (u16, u32) {

    let mut msb = vec![];
    for &b in &line.as_bytes()[12..45] {
        match b {
            b'1' => msb.push(true),
            b'0' => msb.push(false),
            b'|' | b' ' => {}
            _ => panic!("unexpected byte; {:?}", b),
        }
    }

    let lsb = u32::from_str_radix(&line[50..59].trim().to_string(), 16).expect("Invalid hex");
    let len = msb.len() as u16;

    (len, lsb)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use super::*;

    #[test]
    fn parses_huffman_table() { 
        let path = Path::new("assets/hpack-huffman.txt");
        let data = fs::read_to_string(path).expect("Can't read file.");
        let table = parse(&data);

        assert_eq!(table.len(), 257);

        let item = table[10];
        assert_eq!(item.0, 30);
        assert_eq!(item.1, 0x3ffffffc);

        let item = table[32];
        assert_eq!(item.0, 6);
        assert_eq!(item.1, 0x14);

        let item = table.last().unwrap();
        assert_eq!(item.0, 30);
        assert_eq!(item.1, 0x3fffffff);
    }
}
