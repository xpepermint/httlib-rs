> Huffman algorithm for HPACK.

This crate implements static Huffman encoding/decoding algorithm for HTTP/2 used by [HPACK](https://tools.ietf.org/html/rfc7541#appendix-B).

Features:
- `encode`: Enables encoding features (default).
- `decode1`: Enables decoding features for reading 1 bit at a time.
- `decode2`: Enables decoding features for reading 2 bits at a time.
- `decode3`: Enables decoding features for reading 3 bits at a time.
- `decode4`: Enables decoding features for reading 4 bits at a time (default).
- `decode5`: Enables decoding features for reading 5 bits at a time.
- `flatten`: Enables features for flattening Huffman table (default).
- `parse`: Enables features for parsing Huffman table (default).

```toml
[dependencies.httlib-huffman]
...
default-features = false
features = ["encode", "decoder", "decode4"]
```

Usage:
```rs
// Parses HPACK's static Huffman codings.
let codings = httlib_huffman::parse(txt);

/// Builds Huffman transition table for decoding Huffman sequence. The function
/// accepts the `speed` attribute which indicates the number of bits that the
/// decoder should read at the time. Note that increasing the speed, makes the 
/// decoder faster but also contributes to a higher memory footprint. The 
/// suggested value is 4 bits.
let table = httlib_huffman::flatten(codings, speed);

/// Rebuilds Huffman tree and returns the root node.
let tree = httlib_huffman::build(codings);

// Encodes data bytes to Huffman's sequence.
let sequence = httlib_huffman::encode(codings);

/// Decodes Huffman's sequence from the provided table. The table design
/// explains how many bits should be read at the time.
let data = httlib_huffman::decode(sequence, table);
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

Testing:

```bash
$ cargo test --all-features
```

# TODO

* Rename table to table