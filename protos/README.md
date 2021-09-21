# httlib-protos

This crate implements [Protocol Buffers] binary protocol `v3` (`proto3`),
for encoding and decoding typed messages to and from the wire format. The
protocol deals in depth with optimizing the representation of data types on
the wire so that as little data as possible is transmitted between client
and server.

[![Documentation](https://img.shields.io/badge/-Documentation-blue?style=for-the-badge&logo=Rust)](https://docs.rs/httlib-protos)
[![Source](https://img.shields.io/badge/-Source-lightgrey?style=for-the-badge&logo=GitHub)](https://github.com/xpepermint/httlib-rs/tree/main/protos)

### About

Protocol Buffers, also know as `protos` or `protobufs`, is an open-source
interface description language originally developed by Google and a library
that allows JSON-like data messages to be transmitted over the wire without
unnecessary ballast. Today, it is most relevant in the context of [gRPC],
where [RPC] server and client code for arbitrary programming languages is
generated based on Protocol Buffers descriptions.

Protocol Buffers were developed primarily with the goal of speeding up the
transmission of strongly typed key-value message objects over the network,
which in turn means reducing the amount of data that needs to be transmitted
over the wire from A to B.

[REST] and [RPC] are two concepts that are now considered a kind of de facto
way of developing APIs in web development. Communication between the client
and the server is mostly about transferring data in [JSON] format. This is
user-friendly, but highly suboptimal at the network level.

Protocol Buffers addresses this issue and is one of the most optimized
protocols which is also growing in popularity.

This is not a fully featured Protocol Buffers library for generating code
for the client and the server. The crate focuses on the low-level data
compression and decompression for transmiting typed objects over the wire
and offers a full implementation of the Protocol Buffer's binary protocol.
The library can in such be used for different use cases.

### Usage

**Encoding example:**

```rust
use httlib_protos::{Encoder, EncoderLit};

let encoder = Encoder::default();

let mut dst = Vec::new();
encoder.encode((1, 150i32), &mut dst).unwrap(); // common type
encoder.encode((2, EncoderLit::SInt32(-150i32)), &mut dst).unwrap(); // specific type
```

**Decoding example:**

```rust
use httlib_protos::{Decoder, DecoderLit};

let mut decoder = Decoder::default();

let mut buf = vec![0x85, 0x35, 0x85];

let mut dst = vec![];
let size = decoder.decode(&mut buf, &mut dst).unwrap();

for (tag, typ, byt)  in dst {
    if tag == 1 {
        DecoderLit::Int32(byt).parse::<i32>().unwrap();
    }
}
```

### Articles

* [Deep dive into the binary algorithm of Protocol Buffers](https://dev.to/xpepermint/binary-algorithm-in-protocol-buffers-4gl6-temp-slug-7466404)

[Protocol Buffers]: https://en.wikipedia.org/wiki/Protocol_Buffers
[gRPC]: https://grpc.io/
[REST]: https://en.wikipedia.org/wiki/Representational_state_transfer
[RPC]: https://en.wikipedia.org/wiki/Remote_procedure_call
[JSON]: https://en.wikipedia.org/wiki/JSON

License: MIT
