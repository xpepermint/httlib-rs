//! Implements [Canonical Huffman] functionality for handling [HPACK]
//! format in [HTTP/2]. It exposes a simple API for performing the encoding and
//! decoding of [HTTP/2] header string literals according to the [HPACK] spec.
//! 
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [HTTP/2]: https://tools.ietf.org/html/rfc7540
//! [Canonical Huffman]: https://en.wikipedia.org/wiki/Canonical_Huffman_code

pub mod decode;
pub mod encode;
pub mod flatten;
pub mod parse;

pub use decode::*;
pub use encode::*;
pub use flatten::*;
pub use parse::*;
