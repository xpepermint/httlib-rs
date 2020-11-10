//! This crate implements [HPACK], a compression format for efficiently
//! representing HTTP header fields in [HTTP/2]. It exposes a simple API for
//! performing the encoding and decoding of HTTP headers.
//! 
//! [![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-hpack)
//! [![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/hpack)
//! 
//! ## About
//! 
//! [HPACK] introduces a completely new approach to the header packaging and
//! management. Websites today require dozens or hundreds of requests and the
//! redundant header fields in these requests consume bandwidth unnecessarily.
//! Therefore, [HPACK] is a compressor, which's main function is to eliminate
//! redundant header fields. 
//! 
//! The specification is rather short, but as it goes for other [HTTP/2] related
//! specifications, this one is also often unclear and ambiguous, creating
//! numerous issues and uncertainty for implementers. It is also written with an
//! experienced developer in mind and focuses primarily on the decoder
//! functioning and assumes that the implementor will be knowledgeable enough to
//! add all details as he sees are needed for the working product. 
//! 
//! On top of that, a significant shift in thinking is required from the
//! implementer of the [HTTP/2] protocol. A connection in [HTTP/2] is not a
//! single request/response. We can start multiple simultaneous streams in one
//! connection, representing multiple request/response sessions, which was not
//! possible in the previous versions of the HTTP protocol. The [HPACK]
//! compressor uses this characteristic of [HTTP/2] by indexing headers
//! considering the whole connection and not per stream, which might seem
//! somewhat unusual.
//! 
//! ## Usage
//!
//! **Encoding example:**
//! 
//! ```rs
//! use httlib_hpack::Encoder;
//! 
//! let mut encoder = Encoder::default();
//! let mut dst = Vec::new();
//! let name = b":method";
//! let value = b"PATCH";
//! let flags = Encoder::HUFFMAN_VALUE | Encoder::WITH_INDEXING | Encoder::BEST_FORMAT;
//! encoder.encode((name, value, flags), &mut dst)?;
//! ```
//! 
//! **Decoding example:**
//! 
//! ```rs
//! use httlib_hpack::Decoder;
//!
//! let mut decoder = Decoder::default();
//! let mut buf = vec![...];
//! let mut dst = Vec::new();
//! decoder.decode(&mut buf, &mut dst)?;
//! let (name, value, flags) = dst;
//! 
//! if flags & Decoder::NEVER_INDEXED == Decoder::NEVER_INDEXED {
//!     // sensitive header
//! } else {
//!     // common header
//! }
//! ```
//! 
//! ## Articles
//! 
//! * [HPACK: The secret ingredient of HTTP/2](TODO)
//! 
//! [HPACK]: https://tools.ietf.org/html/rfc7541
//! [HTTP/2]: https://tools.ietf.org/html/rfc7540

mod decoder;
mod encoder;
mod table;

pub use decoder::*;
pub use encoder::*;
pub use table::*;
