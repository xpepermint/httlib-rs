> Huffman algorithm for HPACK.

This crate implements static Huffman encoding/decoding algorithm for HTTP/2 used by [HPACK](https://tools.ietf.org/html/rfc7541#appendix-B).

```rs
// Parses HPACK's static Huffman codings.
let codings = httlib_huffman::parse(txt);

/// Builds Huffman transition matrix for decoding Huffman sequence. The function
/// accepts the `speed` attribute which indicates the number of bits that the
/// decoder should read at the time. Note that increasing the speed, makes the 
/// decoder faster but also contributes to a higher memory footprint. The 
/// suggested value is 4 bits.
let matrix = httlib_huffman::flatten(codings, speed);

/// Rebuilds Huffman tree and returns the root node.
let tree = httlib_huffman::build(codings);

// Encodes data bytes to Huffman's sequence.
let sequence = httlib_huffman::encode(codings);

/// Decodes Huffman's sequence from the provided matrix. The matrix design
/// explains how many bits should be read at the time.
let data = httlib_huffman::decode(sequence, matrix);
```

See it in action:

```bash
# Builds the ENCODE_TABLE.
$ cargo run --exeample parse
# Builds the DECODE_TABLE (speed = 2 ... decode 2 bits at a time).
$ cargo run --exeample flatten <speed>
# Encode a sample text (data = "ABCD").
$ cargo run --exeample encode <data>
# Decode a sample sequence (sequence = "0101010001010101").
$ cargo run --exeample decode <sequence>
```
