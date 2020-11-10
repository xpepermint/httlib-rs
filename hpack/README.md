# httlib-hpack

This crate implements [HPACK], a compression format for efficiently
representing HTTP header fields in [HTTP/2]. It exposes a simple API for
performing the encoding and decoding of HTTP headers.

[![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-hpack)
[![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/hpack)

### About

[HPACK] is a compression format that eliminates redundant header fields in
requests and responses. This is one of the features based on which the
[HTTP/2] protocol significantly reduces the amount of transferred data from
one entity to another.

A significant shift in thinking is required from the implementer of the
[HTTP/2] protocol. A connection in [HTTP/2] does not represent a single
request/response session. We can start multiple simultaneous streams in one
connection, representing multiple request/response sessions, which was not
possible in the previous versions of the HTTP protocol. The [HPACK]
compressor uses this characteristic of [HTTP/2] by indexing headers
considering the whole connection and not per stream.

The implementation of [HPACK] contains three main parts of the process:

* `Indexing table` is a list, to which the HPACK saves the commonly used
headers. Each entity indexes headers per connection, separately for incoming
(decoding) and for outgoing (encoding) data ([2.3.]).

* `Encoder` TODO!!!!!

* `Decoder` TODO!!!!!

### Usage

**Encoding example:**

```rs
use httlib_hpack::Encoder;

let mut encoder = Encoder::default();
let mut dst = Vec::new();
let name = b":method";
let value = b"PATCH";
let flags = Encoder::HUFFMAN_VALUE | Encoder::WITH_INDEXING | Encoder::BEST_FORMAT;
encoder.encode((name, value, flags), &mut dst)?;
```

**Decoding example:**

```rs
use httlib_hpack::Decoder;

let mut decoder = Decoder::default();
let mut buf = vec![...];
let mut dst = Vec::new();
decoder.decode(&mut buf, &mut dst)?;
let (name, value, flags) = dst;

if flags & Decoder::NEVER_INDEXED == Decoder::NEVER_INDEXED {
    // sensitive header
} else {
    // common header
}
```

### Articles

* [HPACK: The secret ingredient of HTTP/2](TODO)

[HPACK]: https://tools.ietf.org/html/rfc7541
[HTTP/2]: https://tools.ietf.org/html/rfc7540
[2.3.]: https://tools.ietf.org/html/rfc7541#section-2.3

License: MIT
