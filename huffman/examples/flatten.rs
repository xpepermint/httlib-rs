extern crate httlib_huffman;

use httlib_huffman::{flatten};
use httlib_huffman::data::encoder::ENCODE_TABLE;

fn main() {
    let matrix = flatten(&ENCODE_TABLE, 4);

    println!("");
/// This is a static translation matrix for decoding Huffman sequence by reading
/// 2-bits at at time.
/// 
/// The result is a list of tuples where thest first
    println!("pub const ENCODE_TABLE: [(isize, i8, u8); 311] = [");
    for levels in matrix {
        println!("  [");
        // for (next_id, ascii, leftovers) in levels {
        //     println!("      ({}, {}, {}),", next_id, ascii, leftovers);
        // }
        println!("  ],");
    }
    println!("];");
    println!("");
}
