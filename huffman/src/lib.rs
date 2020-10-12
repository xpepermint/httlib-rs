//! This module implementing [Canonical Huffman] functionality handling [HPACK]
//! format in [HTTP/2]. It exposes a simple API for performing the encoding and
//! decoding of [HTTP/2] header string literals according to the [HPACK] spec.
//! 
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [HTTP/2]: https://tools.ietf.org/html/rfc7540
//! [Canonical Huffman]: https://en.wikipedia.org/wiki/Canonical_Huffman_code

mod builder;
mod decoder;
mod encoder;
mod flattener;
mod parser;
pub mod data;

pub use builder::*;
pub use decoder::*;
pub use encoder::*;
pub use flattener::*;
pub use parser::*;
