//! This crate implements [canonical Huffman] functionality for handling [HPACK]
//! format in [HTTP/2]. It exposes a simple API for performing the encoding and
//! decoding of [HTTP/2] header string literals according to the [HPACK] spec.
//! 
//! [![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-huffman)
//! [![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/huffman)
//! 
//! Header Compression format for [HTTP/2], known as [HPACK], foresees the use
//! of the Huffman algorithm for encoding header literal values. This
//! contributes to the additional decrease in the quantity of data, transferred
//! with each web request and response.
//! 
//! A [Huffman code] is a particular type of optimal prefix code that is
//! commonly used for lossless data compression. The process of finding or using
//! such a code proceeds by means of Huffman coding, an algorithm developed by
//! David A. Huffman. The output from Huffman's algorithm can be viewed as a
//! variable-length code table for encoding a source symbol (such as a character
//! in a file). The algorithm derives this table from the estimated probability
//! or frequency of occurrence (weight) for each possible value of the source
//! symbol. As in other entropy encoding methods, more common symbols are
//! generally represented using fewer bits than less common symbols. Huffman's
//! method can be efficiently implemented, finding a code in time linear to the
//! number of input weights if these weights are sorted.
//! 
//! [HPACK] compression entails a pre-created [canonical Huffman] code table
//! for encoding [ASCII] characters to the Huffman sequence. A
//! [canonical Huffman] code is a particular type of [Huffman code] with unique
//! properties that allow it to be described in a very compact manner. In the
//! aforementioned table are the Huffman codes for each [ASCII] character with a
//! length up to 32 bits (4x by 8 fields with value 0 or 1), in the form of
//! base-2 integer, aligned on the most significant bit (MSB is the bit farthest
//! to the left).
//! 
//! Each module covers this topic in more details so check the rest of the code
//! to learn more.
//! 
//! ## Usage
//!
//! **Encoding example:**
//! 
//! ```rust
//! use httlib_huffman::encode;
//! 
//! let mut dst = Vec::new();
//! let text = "Hello world!".as_bytes();
//! encode(&text, &mut dst).unwrap();
//! ```
//! 
//! **Decoding example:**
//! 
//! ```rust
//! use httlib_huffman::{DecoderSpeed, decode};
//!
//! let speed = DecoderSpeed::ThreeBits;
//! let mut dst = Vec::new();
//! let src = vec![135];
//! decode(&src, &mut dst, speed).unwrap();
//! ```
//! 
//! ## Articles
//! 
//! * [HPACK: Huffman encoder](https://dev.to/xpepermint/hpack-huffman-encoder-3i7c)
//! * [HPACK: Huffman translation matrix](https://dev.to/xpepermint/hpack-huffman-translation-matrix-64c)
//! * [HPACK: Huffman decoder](https://dev.to/xpepermint/hpack-huffman-decoder-52el)
//! 
//! [ASCII]: https://en.wikipedia.org/wiki/ASCII
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [HTTP/2]: https://tools.ietf.org/html/rfc7540
//! [Huffman code]: https://en.wikipedia.org/wiki/Huffman_coding
//! [canonical Huffman]: https://en.wikipedia.org/wiki/Canonical_Huffman_code

pub mod decoder;
pub mod encoder;
pub mod flattener;
pub mod parser;

pub use decoder::*;
pub use encoder::*;
