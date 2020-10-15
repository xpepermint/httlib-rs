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
    println!("pub const DECODE_TABLE: [[(Option<usize>, Option<usize>, usize); 16]; 54] = [");
    for (i, transitions) in matrix.iter().enumerate() {
        println!("    [ // {}", i);

        for (j, transition) in transitions.iter().enumerate() {
            print!("        {:?}", transition);
            if j != 15 {
                print!(",");
            }
            println!("");
        }
        print!("    ]");
        if i != 53 {
            print!(",");
        }
        println!("");
    }
    println!("];");
    println!("");
}
