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
    #[test]
    fn parses_table() {
        // TODO
    }
}
