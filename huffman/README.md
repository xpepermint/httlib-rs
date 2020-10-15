> Canonical Huffman algorithm for handling HPACK format in HTTP/2.

[Documentation]

This crate implements [canonical Huffman] functionality for handling [HPACK]
format in [HTTP/2]. It exposes a simple API for performing the encoding and
decoding of [HTTP/2] header string literals according to the [HPACK] spec.

Header Compression format for [HTTP/2], known as [HPACK], foresees the use
of the Huffman algorithm for encoding header literal values. This
contributes to the additional decrease in the quantity of data, transferred
with each web request and response.

A [Huffman code] is a particular type of optimal prefix code that is
commonly used for lossless data compression. The process of finding or using
such a code proceeds by means of Huffman coding, an algorithm developed by
David A. Huffman. The output from Huffman's algorithm can be viewed as a
variable-length code table for encoding a source symbol (such as a character
in a file). The algorithm derives this table from the estimated probability
or frequency of occurrence (weight) for each possible value of the source
symbol. As in other entropy encoding methods, more common symbols are
generally represented using fewer bits than less common symbols. Huffman's
method can be efficiently implemented, finding a code in time linear to the
number of input weights if these weights are sorted.

[HPACK] compression entails a pre-created [canonical Huffman] code table
for encoding [ASCII] characters to the Huffman sequence. A
[canonical Huffman] code is a particular type of [Huffman code] with unique
properties that allow it to be described in a very compact manner. The
advantage of a [canonical Huffman] tree is that one can encode data in fewer
bits than with a fully described tree. In the aforementioned table are the
Huffman codes for each [ASCII] character with a length up to 32 bits (4x by
8 fields with value 0 or 1), in the form of base-2 integer, aligned on the
most significant bit (MSB is the bit farthest to the left).

[ASCII]: https://en.wikipedia.org/wiki/ASCII
[HPACK]: https://tools.ietf.org/html/rfc7541
[HTTP/2]: https://tools.ietf.org/html/rfc7540
[Huffman code]: https://en.wikipedia.org/wiki/Huffman_coding
[canonical Huffman]: https://en.wikipedia.org/wiki/Canonical_Huffman_code
[Documentation]: https://docs.rs/httlib-huffman
