//! The example will print out the source code for the DECODE_TABLE constant
//! which is provided by this crate through the `decode::table4` module.

extern crate httlib_huffman;

use httlib_huffman::{flatten, DecoderSpeed};
use httlib_huffman::encoder::table::ENCODE_TABLE;

fn main() {
    let speed = DecoderSpeed::FourBits;
    let table = flatten(&ENCODE_TABLE, speed);
    let table_len = table.len();
    let targets_len = table[0].len();

    println!("");
    println!("/// This is a static translation table for decoding Huffman sequence by reading");
    println!("/// {:?}-bit(s) at at time.", speed);
    println!("pub const DECODE_TABLE: [[(Option<u8>, Option<u16>, u8); {}]; {}] = [ // (next_id, ascii, leftover)", targets_len, table_len);
    for (i, transitions) in table.iter().enumerate() {
        println!("    [ // {}", i);

        for (j, transition) in transitions.iter().enumerate() {
            print!("        {:?}", transition);
            if j != targets_len - 1 {
                print!(",");
            }
            println!("");
        }
        print!("    ]");
        if i != table_len - 1 {
            print!(",");
        }
        println!("");
    }
    println!("];");
    println!("");
}
