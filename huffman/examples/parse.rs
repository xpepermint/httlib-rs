extern crate httlib_huffman;

use std::fs;
use std::path::Path;
use httlib_huffman::{parse};

fn main() {
    let path = Path::new("assets/hpack-huffman.txt");
    let data = fs::read_to_string(path).expect("Can't read file.");
    let codings = parse(&data);

    println!("");
    println!("/// This is a static Huffman table built from the codes found in the official");
    println!("/// HPACK specification (Appendix B).");
    println!("/// ");
    println!("/// The result is a list of tuples where the first item represents the number of");
    println!("/// bits for the code representing the symbol and the second is the Huffman code");
    println!("/// for the symbol represented as a base-2 integer, aligned on the most");
    println!("/// significant bit (MSB).");
    println!("pub const ENCODE_TABLE: [(u8, u32); 257] = [");
    for coding in codings.iter() {
        println!("  ({}, 0x{:02x}),", coding.0, coding.1);
    }
    println!("];");
    println!("");
}
