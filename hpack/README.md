# httlib-hpack

This crate implements [HPACK], a compression format for efficiently
representing HTTP header fields in [HTTP/2]. It exposes a simple API for
performing the encoding and decoding of HTTP headers.

[![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-hpack)
[![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/hpack)

### Usage

Encoding:

```rs
use httlib_hpack::Encoder;

let mut sequence = Vec::new();
let mut encoder = Encoder::default();
encoder.encode(b":method", b"GET")?;
```

Decoding:

```rs
use httlib_hpack::Decoder;

let mut dst = Vec::new();
let mut src = vec![168, 209, ...];
let mut decoder = Decoder::default();
decoder.decode(&mut src, &mut dst)?;
```

### Articles

* [HPACK: The secret ingredient of HTTP/2](TODO)

[HPACK]: https://tools.ietf.org/html/rfc7541
[HTTP/2]: https://tools.ietf.org/html/rfc7540

License: MIT
