//! This crate implements [HPACK], a compression format for efficiently
//! representing HTTP header fields in [HTTP/2]. It exposes a simple API for
//! performing the encoding and decoding of HTTP headers.
//! 
//! [![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-hpack)
//! [![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/hpack)
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
